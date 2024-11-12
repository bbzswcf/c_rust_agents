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

from tree_sitter_c_config import c_parser
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

def extract_func_calls(file_relapath: str, func_name: str, metadata: dict):
    """
    Extract function calls from C code(function-level) using tree-sitter.
    """
    c_func_calls = []

    for func_info in metadata[file_relapath]['functions']:
        if func_info['name'] == func_name:
            for depend_func in func_info['depend_funcs']:
                c_func_calls.append(depend_func['name'])

    return c_func_calls

def dependencies_order(func_name, file_relapath, metadata):
    total = 0
    depend_funcs = []
    for func_info in metadata[file_relapath]['functions']:
        if func_info['name'] == func_name:
            
            # Recursively count dependencies of dependent functions
            for func in func_info['depend_funcs']:
                total, funcs = dependencies_order(func['name'], func['file'], metadata)
                depend_funcs.extend(funcs)
                depend_funcs.append((func['name'], func['file']))
            total += len(func_info['depend_funcs'])
            break
    return total, depend_funcs

def sort_by_depend_count(test_funcs, file_relapath, metadata):
    """
    Sort functions by the number of dependencies they have.
    """
    test_func_counts = []
    for test_func in test_funcs:
        count, funcs = dependencies_order(test_func, file_relapath, metadata)
        test_func_counts.append((test_func, count))
    
    print(test_func_counts)
    sorted_funcs = sorted(test_func_counts, key=lambda x: x[1], reverse=False)
    return [func for func, _ in sorted_funcs]


def extract_func_dependencies(directory, test_file, func):
    """
    Extract dependencies of a specific function from a C file.
    """
    # Get all header files in directory
    header_files = []
    for root, dirs, files in os.walk(directory):
        for file in files:
            if file.endswith('.h'):
                if os.path.splitext(file)[0] not in EXCEPT_FILES:
                    header_files.append(file)

    test_total_path = os.path.join(directory, test_file)

    # Find all include directives in the test file 
    with open(test_total_path, 'r') as f:
        tree = c_parser.parse(bytes(f.read(), 'utf-8'))

    includes = []
    include_nodes = tree.root_node.children_by_field_name('include')
    for node in include_nodes:
        include_text = node.text.decode('utf-8')

        if '"' in include_text:
            include_name = include_text[include_text.find('"')+1:include_text.rfind('"')]
        elif '<' in include_text:
            include_name = include_text[include_text.find('<')+1:include_text.rfind('>')]

        if include_name in header_files:
            includes.append(include_name)

    # Remove duplicates from includes while preserving order
    includes = list(dict.fromkeys(includes))

    # Extract the function code for the specified function name
    func_code = None
    for node in tree.root_node.children:
        if node.type == 'function_definition':
            func_decl = node.child_by_field_name('declarator')
            if func_decl.type == 'function_declarator':
                func_name = func_decl.child_by_field_name('declarator')
            elif func_decl.type == 'pointer_declarator':
                func_name = func_decl.children[1].child_by_field_name('declarator')

            if func_name and func_name.text.decode('utf-8') == func:
                func_code = node.text.decode('utf-8')
                break

    # Build mapping of include file to function declarations
    include_functions = {}

    for include in includes:
        header_path = None
        for root, dirs, files in os.walk(directory):
            if include in files:
                header_path = os.path.join(root, include)
                break

        with open(header_path, 'r') as f:
            tree = c_parser.parse(bytes(head_preprocess(f.read()), 'utf-8'))

        funcs = []
        # Find all function declarations in the header
        for node in tree.root_node.children:
            if node.type == 'declaration':
                func_decl = node.child_by_field_name('declarator')
                if func_decl and func_decl.type == 'function_declarator':
                    func_name = func_decl.child_by_field_name('declarator')
                elif func_decl and func_decl.type == 'pointer_declarator':
                    func_name = func_decl.children[1].child_by_field_name('declarator')
                if func_name:
                    funcs.append(func_name.text.decode('utf-8'))

        include_functions[include] = funcs

    # Find functions from include files that are used in func_code
    used_functions = {}
    if func_code:
        for include, funcs in include_functions.items():
            used = []
            for func_name in funcs:
                if func_name in func_code:
                    used.append(func_name)

            used_functions[os.path.splitext(include)[0]] = used

    return used_functions
    

def head_info_extraction(directory, file):
    """
    Extract non-function declarations (like structs, type definitions, etc.) from header files.
    Returns a dictionary where key is include filename and value is the non-function declaration content.
    """
    # Locate target C file path
    file_path = None 
    file = file + '.c'
    for root, dirs, files in os.walk(directory):
        if file in files:
            file_path = os.path.join(root, file)
            break
    
    # Read and preprocess C file content
    with open(file_path, 'r', encoding='utf-8') as f:
        code = preprocess(f.read())
    
    # Parse C file to get all include statements
    tree = c_parser.parse(bytes(code, 'utf8'))
    includes = []
    for node in tree.root_node.children:
        if node.type == 'preproc_include':
            for child in node.children:
                if child.type == 'string_literal':
                    include_path = child.text.decode('utf8').strip('"')
                    if os.path.splitext(include_path)[0] not in EXCEPT_FILES:
                        includes.append(include_path)
                    
    # Traverse each header file to extract non-function declarations
    header_info = {}
    for include in includes:
        header_path = None
        for root, _, files in os.walk(directory):
            if include in files:
                header_path = os.path.join(root, include)
                break
                
        if header_path:
            # Read and preprocess header file
            with open(header_path, 'r', encoding='utf-8') as f:
                header_code = head_preprocess(f.read())
                
            header_tree = c_parser.parse(bytes(header_code, 'utf8'))
            
            # Traverse declarations in header file to find the first function declaration
            # Content before function declarations are non-function declarations (structs, type definitions, etc.)
            for node in header_tree.root_node.children:
                if node.type == 'declaration':
                    declarator = node.child_by_field_name('declarator')
                    if declarator and (declarator.type == 'function_declarator' or 
                                     (declarator.type == 'pointer_declarator' and 
                                      len(declarator.children) > 1 and
                                      declarator.children[1].type == 'function_declarator')):
                        start_byte = node.start_byte
                        code = header_code[:start_byte].strip()

                        header_info[include] = code
                        break
                
    return header_info


if __name__ == "__main__":
    # Below are tests for various functionalities
    parser = argparse.ArgumentParser(description='Analyze C code dependencies and suggest translation order.')
    parser.add_argument('--directory', default='./Input/01-Primary', type=str, help='Directory path to analyze')
    args = parser.parse_args()

    with open('tool/c_metadata.json', 'r', encoding='utf-8') as f:
        metadata = json.load(f)
    dependencies = analyze_directory(metadata)

    test_funcs = extract_test_functions("test\\test-hash-functions.c", metadata)
    test_funcs = sort_by_depend_count(test_funcs, "test\\test-hash-functions.c", metadata)

    # # Print files and their dependencies
    # print("\nFiles and their dependencies:")
    # for file, deps in dependencies.items():
    #     print(f"  {file}: {deps}")

    # # Get and print the suggested translation order
    # translation_order = get_translation_order(dependencies)
    # print("\nSuggested translation order:")
    # count = 1
    # for i, file in enumerate(translation_order, 1):
    #     print(f"{count}. {file}")
    #     count += 1

    # test_funcs = extract_test_functions("test\\test-arraylist.c", metadata)
    # print(test_funcs)

    # includes = extract_func_dependencies(args.directory, "test\\test-arraylist.c", 'test_arraylist_append')
    # print(includes)
    # head_info = head_info_extraction(args.directory, "test-arraylist.c")
    # print(head_info)
    # Test extract_func_calls
    # Test extract_all_funcs

    # funcs = extract_all_funcs("arraylist")
    # print("Found functions:", funcs)
    
    # # Test saving parse tree for rb-tree.c in a more readable format
    # rb_tree_path = os.path.join(args.directory, "src/rb-tree.c")
    # with open(rb_tree_path, 'r', encoding='utf-8') as f:
    #     tree = c_parser.parse(bytes(f.read(), 'utf-8'))
    
    # def print_tree(node, level=0):
    #     # Create indentation based on level
    #     indent = "  " * level
        
    #     # Print current node type and text if it's a token
    #     if len(node.children) == 0:  # It's a token
    #         return f"{indent}{node.type}: {node.text.decode('utf-8')}\n"
    #     else:
    #         result = f"{indent}{node.type}"
    #         if node.text:
    #             result += f": {node.text.decode('utf-8')}"
    #         result += "\n"
    #         # Recursively print all children
    #         for child in node.children:
    #             result += print_tree(child, level + 1)
    #         return result
            
    # # Save the tree in a more readable format
    # with open('rb_tree_ast.txt', 'w', encoding='utf-8') as f:
    #     f.write(print_tree(tree.root_node))
    
    # print(f"Saved readable rb-tree.c AST to rb_tree_ast.txt")
    # Test extract_func_calls
    # test_code = """
    # void test_func() {
    #     int x = add(1, 2);
    #     printf("Result: %d\n", x);
    #     char* str = malloc(10);
    #     strcpy(str, "test");
    #     free(str);
    #     convert_to_rust();
    #     convert_c_to_rust(str);
    # }
    # """
    # func_calls = extract_func_calls(test_code)
    # print("\nTesting extract_func_calls:")
    # print("Input code:")
    # print(test_code)
    # print("\nExtracted function calls:", func_calls)
    # 输入字符串
