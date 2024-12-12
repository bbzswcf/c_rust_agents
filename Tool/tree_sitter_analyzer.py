"""
This script analyzes C source files and their dependencies in a given directory.
It utilizes the tree-sitter library for parsing C code and extracting dependencies.
The script implements a custom topological sort algorithm to order the dependencies.
Output includes a list of files found and the topologically sorted list of files.
"""
import os
import re
import json
import argparse

from tree_sitter_c_config import c_parser, rust_parser
from c_code_preprocess import preprocess, head_preprocess

# Files to exclude from dependency analysis
# These are common test framework files that don't need translation
EXCEPT_FILES = ['framework', 'alloc-testing', 'test-alloc-testing']

def add_to_translation_order(dependencies, current_key, translation_order):
    """
    Add a file and its dependencies to the translation order list.
    """
    if current_key in translation_order:
        return
        
    deps = dependencies.get(current_key, [])
    
    if not deps:
        translation_order.append(current_key)
    else:
        all_deps_processed = all(dep in translation_order for dep in deps)
        
        if all_deps_processed:
            last_dep_pos = max(translation_order.index(dep) for dep in deps)
            translation_order.insert(last_dep_pos + 1, current_key)
        else:
            for dep in deps:
                if dep not in translation_order:
                    add_to_translation_order(dependencies, dep, translation_order)
            translation_order.append(current_key)

def get_translation_order(dependencies):
    """
    Generate a topological order of files based on their dependencies.
    This function processes test files first by adding them and their dependencies
    to the translation order list.
    """
    translation_order = []
    for file, deps in dependencies.items():
        if file.startswith('test'):
            add_to_translation_order(dependencies, file, translation_order)
    for file in translation_order:
        if file.startswith('src'):
            translation_order[translation_order.index(file)] = file + '.c'
    return translation_order

def analyze_directory(metadata):
    """
    Analyze C files and extract their dependencies based on #include directives.
    """
    dependencies = {}

    c_file_names = []
    for file in metadata.keys():
        c_file_names.append(os.path.splitext(os.path.basename(file))[0])
    c_file_names = list(dict.fromkeys(c_file_names))

    for c_file, file_info in metadata.items():
        if not file_info['includes']:
            dependencies[c_file] = []
        else:
            for include in file_info['includes']:
                include_code = include['code']
                match = re.search(r'#include\s*[<"]([^>"]+)[>"]', include_code)
                header_name = match.group(1)
                header_base = os.path.splitext(header_name)[0]
                if c_file not in dependencies:
                    dependencies[c_file] = []
                if header_base in c_file_names and header_base != os.path.splitext(os.path.basename(c_file))[0]:
                    if header_base.startswith('test'):
                        dependencies[c_file].append(os.path.join('test', header_base))
                    else:
                        dependencies[c_file].append(os.path.join('src', header_base))
    
    return dependencies

def extract_test_functions(file, metadata):
    """
    Extract test functions from a C test file and return them in the order they appear.
    """
    test_functions = []
    
    for func_info in metadata[file]['functions']:
        if func_info['name'].startswith('test_'):
            test_functions.append(func_info['name'])

    return test_functions

def dependencies_order(func_name, file_relapath, metadata):
    depend_funcs = []
    for func_info in metadata[file_relapath]['functions']:
        if func_info['name'] == func_name:
            
            # Recursively count dependencies of dependent functions
            for func in func_info['depend_funcs']:
                funcs = dependencies_order(func['name'], func['file'], metadata)
                depend_funcs.extend(funcs)
                depend_funcs.append((func['name'], func['file']))
            break
    depend_funcs = list(dict.fromkeys(depend_funcs))
    return depend_funcs

def sort_by_depend_count(test_funcs, file_relapath, metadata):
    """
    Sort functions by the number of dependencies they have.
    """
    test_func_counts = []
    for test_func in test_funcs:
        funcs = dependencies_order(test_func, file_relapath, metadata)
        test_func_counts.append((test_func, len(funcs)))

    sorted_funcs = sorted(test_func_counts, key=lambda x: x[1], reverse=False)
    return [func for func, _ in sorted_funcs]

def extract_rust_funcs(code: str) -> tuple[list, list]:
    """
    Extract all function definitions and use statements from Rust code using tree-sitter.
    Returns a tuple of (function definitions list, use statements list).
    """
    funcs = []
    uses = []
    tree = rust_parser.parse(bytes(code, 'utf8'))
    
    def traverse_node(node, code, funcs, uses):
        if node.type == 'function_item':
            start_byte = node.start_byte
            end_byte = node.end_byte
            func_code = code[start_byte:end_byte]
            funcs.append(func_code)
        elif node.type == 'use_declaration':
            start_byte = node.start_byte
            end_byte = node.end_byte
            use_code = code[start_byte:end_byte]
            uses.append(use_code)
        else:
            for child in node.children:
                traverse_node(child, code, funcs, uses)

    for node in tree.root_node.children:
        traverse_node(node, code, funcs, uses)
                
    return funcs, uses
