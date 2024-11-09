"""
This module provides functionality for preprocessing C code.
It includes functions to remove copyright notices.

Note:
    Additional preprocessing functionality is pending implementation.
    Future improvements may include:
    - Macro expansions
    - Removal of useless comments
    - Handling of conditional compilation directives (#ifdef, #ifndef, etc.)
    - Other C-specific preprocessing tasks

TODO:
    Implement additional preprocessing steps to enhance the robustness
    and completeness of the C code preprocessing.
"""
import re
import os
import json
import argparse

from tree_sitter_c_config import c_parser

EXCEPT_FILES = ['framework', 'alloc-testing', 'test-alloc-testing']

# 头文件处理
def remove_file_comments(code: str) -> str:
    """
    Remove all C-style comments from code.
    """
    pattern = r'/\*.*?\*/\s*'
    preprocessed_code = re.sub(pattern, '', code, flags=re.DOTALL)
    pattern = r'\n\s*\n'
    preprocessed_code = re.sub(pattern, '\n', preprocessed_code)

    return preprocessed_code.lstrip()

def remove_header_guards(code: str) -> str:
    """
    Remove header include guards (#ifndef/#define/#endif) but keep their contents.
    """
    # Match the entire header guard pattern
    pattern = r'#ifndef\s+[A-Z_]+\s*\n\s*#define\s+[A-Z_]+\s*\n(.*)\s*#endif\s*$'
    
    # Replace with just the contents (captured group 1)
    preprocessed_code = re.sub(pattern, r'\1', code, flags=re.DOTALL)
    return preprocessed_code

def remove_cpp_guards(code: str) -> str:
    """
    Remove C++ extern "C" guards (#ifdef __cplusplus) and their contents.
    """
    pattern = r'\s*#ifdef\s+__cplusplus\s*\n.*?#endif\s*\n'
    preprocessed_code = re.sub(pattern, '', code, flags=re.DOTALL)
    return preprocessed_code

# 源文件处理
def remove_copyright(code: str) -> str:
    """
    Remove copyright notices from C code.
    """
    copyright_pattern = r'^/\*(?:(?!\*/).)*Copyright(?:(?!\*/).)*\*/'
    code = re.sub(copyright_pattern, '', code, flags=re.DOTALL | re.MULTILINE)
    return code.lstrip()

def process_alloc_testing(code: str) -> str:
    """
    Process allocation testing related code by:
    - Removing #ifdef ALLOC_TESTING blocks
    - Removing malloc/free testing comments
    """
    pattern = r'\n?\s*#ifdef\s+ALLOC_TESTING.*?#endif\n?'
    code = re.sub(pattern, '', code, flags=re.DOTALL)
    
    pattern = r'/\*\s*malloc\(\)\s*/\s*free\(\)\s*testing\s*\*/\s*\n?'
    code = re.sub(pattern, '', code)

    return code

# 测试文件处理
def remove_test_includes(code: str) -> str:
    """
    Remove test framework and alloc testing includes from test files.
    """
    pattern = r'#include\s+"framework\.h"\s*\n'
    code = re.sub(pattern, '', code)
    
    pattern = r'#include\s+"alloc-testing\.h"\s*\n'
    code = re.sub(pattern, '', code)
    
    return code

def remove_debug_blocks(code: str) -> str:
    """
    Remove debug blocks from C code.
    """
    pattern = r'\n?\s*#if\s+0\s*\n(.*?)\n\s*#endif\n?'
    preprocessed_code = re.sub(pattern, '\n', code, flags=re.DOTALL | re.MULTILINE)
    return preprocessed_code

def remove_alloc_test_blocks(code: str) -> str:
    """
    Remove alloc_test blocks from test files, keeping only the closing brace.
    """
    start_idx = code.find('alloc_test')
    if start_idx == -1:
        return code
        
    end_idx = code.rfind('}')
    if end_idx == -1:
        return code
        
    # Keep everything before alloc_test and after the last brace
    return code[:start_idx] + code[end_idx:]

def init_metadata(metadata: dict, relative_path: str):
    metadata[relative_path] = {}
    metadata[relative_path]['func_signatures'] = []
    metadata[relative_path]['private_func_signatures'] = []
    metadata[relative_path]['public_func_signatures'] = []
    metadata[relative_path]['includes'] = []
    metadata[relative_path]['head_info'] = []
    metadata[relative_path]['variables'] = []
    metadata[relative_path]['functions'] = []

def preprocess(code: str) -> str:
    code = remove_file_comments(code)
    code = remove_header_guards(code)
    code = remove_cpp_guards(code)
    return code

def head_preprocess(code: str) -> str:
    code = remove_file_comments(code)
    code = remove_header_guards(code) 
    code = remove_cpp_guards(code)
    return code

def code_preprocess(directory: str):
    head_file_list = []
    src_file_list = []
    test_file_list = []

    for root, dirs, files in os.walk(directory):
        for file in files:
            if not file.endswith('.c') and not file.endswith('.h') and not file.endswith('.cpp'):
                continue
            if os.path.splitext(file)[0] in EXCEPT_FILES:
                continue
            total_path = os.path.join(root, file)
            if file.endswith('.h'):
                head_file_list.append(total_path)
            elif file.startswith('test-'):
                test_file_list.append(total_path)
            else:
                src_file_list.append(total_path)

    metadata = {}

    for total_path in head_file_list:
        relative_path = os.path.relpath(total_path, directory)
        with open(total_path, 'r', encoding='utf-8') as f:
            code = f.read()

        relative_path = os.path.splitext(relative_path)[0] + '.c'

        if relative_path not in metadata:
            init_metadata(metadata, relative_path)

        code = remove_file_comments(code)
        code = remove_header_guards(code)
        code = remove_cpp_guards(code)

        tree = c_parser.parse(bytes(code, 'utf-8'))
        for node in tree.root_node.children:
            if node.type == 'declaration':
                public_func_signature = ' '.join(node.text.decode('utf-8').replace(';', '').split()).strip()
                metadata[relative_path]['public_func_signatures'].append(public_func_signature)
            else:
                if node.text.decode('utf-8') != ';':
                    metadata[relative_path]['head_info'].append({'code': node.text.decode('utf-8')})
    
    for total_path in src_file_list:
        relative_path = os.path.relpath(total_path, directory)

        if relative_path not in metadata:
            init_metadata(metadata, relative_path)
        
        with open(total_path, 'r', encoding='utf-8') as f:
            code = f.read()
        
        code = remove_copyright(code)
        code = process_alloc_testing(code)

        tree = c_parser.parse(bytes(code, 'utf-8'))
        for node in tree.root_node.children:
            if node.type == 'comment':
                continue
            if node.type == 'preproc_include':
                metadata[relative_path]['includes'].append({'code': node.text.decode('utf-8').strip()})
            elif node.type == 'function_definition':
                func_signature = ' '.join(node.text.decode('utf-8').split('{')[0].split()).strip()
                func_name = func_signature.split('(')[0].split()[-1].replace('*', '')

                metadata[relative_path]['functions'].append({'name': func_name,'signature': func_signature, 'code': node.text.decode('utf-8')})
                metadata[relative_path]['func_signatures'].append(func_signature)
                if func_signature not in metadata[relative_path]['public_func_signatures']:
                    metadata[relative_path]['private_func_signatures'].append(func_signature)
            else:
                if node.text.decode('utf-8') != ';':
                    metadata[relative_path]['variables'].append({'code': node.text.decode('utf-8')})
    
    for total_path in test_file_list:
        relative_path = os.path.relpath(total_path, directory)

        if relative_path not in metadata:
            init_metadata(metadata, relative_path)
        
        with open(total_path, 'r', encoding='utf-8') as f:
            code = f.read()

        code = remove_copyright(code)
        code = remove_debug_blocks(code)
        code = remove_test_includes(code)

        # 去掉和alloc_test相关的代码
        tree = c_parser.parse(bytes(code, 'utf-8'))
        memory_funcs_startbyte = []
        memory_funcs_endbyte = []
        code = ''
        for node in tree.root_node.children:
            func_code = node.text.decode('utf-8')
            if node.type == 'function_definition':
                if func_code.startswith('int main'):
                    continue
                func_signature = func_code.split('\n')[0]
                if 'out_of_memory' in func_signature:
                    continue
                else:
                    code = code + remove_alloc_test_blocks(func_code) + '\n\n'
            else:
                if not func_code.startswith('static UnitTestFunction tests'):
                    code =code + func_code + '\n\n'
        code = code.strip()


        tree = c_parser.parse(bytes(code, 'utf-8'))
        for node in tree.root_node.children:
            if node.type == 'comment':
                continue
            if node.type == 'preproc_include':
                metadata[relative_path]['includes'].append({'code': node.text.decode('utf-8')})
            elif node.type == 'function_definition':
                func_signature = ' '.join(node.text.decode('utf-8').split('{')[0].split()).strip()
                func_name = func_signature.split('(')[0].split()[-1].replace('*', '')

                metadata[relative_path]['functions'].append({'name': func_name,'signature': func_signature, 'code': node.text.decode('utf-8')})
                metadata[relative_path]['func_signatures'].append(func_signature)
            else:
                metadata[relative_path]['variables'].append({'code': node.text.decode('utf-8')})
    
    with open('tool/c_metadata.json', 'w', encoding='utf-8') as f:
        json.dump(metadata, f, ensure_ascii=False, indent=2)


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description='Preprocess C code.')
    parser.add_argument('--directory', default='./Input/01-Primary', type=str, help='Directory path to preprocess')
    args = parser.parse_args()

    code_preprocess(args.directory)

