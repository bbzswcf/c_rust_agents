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
    pattern = r'//.*?\n'
    preprocessed_code = re.sub(pattern, '\n', preprocessed_code)
    pattern = r'\n\s*\n'
    preprocessed_code = re.sub(pattern, '\n', preprocessed_code)

    return preprocessed_code.strip()

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

def is_function_declaration(node) -> bool:
    """
    使用 tree-sitter 语法树检查节点是否为函数声明
    """
    # 检查节点类型
    if node.type == 'declaration':
        # 获取声明的子节点
        for child in node.children:
            # 检查是否包含函数声明符
            if child.type == 'function_declarator':
                return True
            # 检查是否包含函数指针
            if child.type == 'pointer_declarator':
                if any(subchild.type == 'function_declarator' 
                      for subchild in child.children):
                    return True
    return False

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
    # Find the line containing alloc_test
    lines = code.split('\n')
    start_idx = -1
    for i, line in enumerate(lines):
        if 'alloc_test' in line:
            # Get character index of start of this line
            start_idx = sum(len(l) + 1 for l in lines[:i])
            break
            
    if start_idx == -1:
        return code

    # Find the first standalone closing brace after the alloc_test line
    brace_count = 0
    end_idx = -1
    for i in range(start_idx, len(code)):
        if code[i] == '{':
            brace_count += 1
        elif code[i] == '}':
            if brace_count == 0:
                end_idx = i
                break
            else:
                brace_count -= 1

    if end_idx == -1:
        return code

    # Keep everything before alloc_test and after the first standalone closing brace
    return code[:start_idx] + code[end_idx:]

def extract_func_calls(c_code: str):
    """
    Extract function calls from C code, including functions used as parameters.
    Returns two lists: regular function calls and functions used as parameters.
    """
    tree = c_parser.parse(bytes(c_code, "utf8"))
    root_node = tree.root_node

    c_func_calls = []
    func_as_params = []

    def traverse_node(node):
        if node.type == 'call_expression':
            # Get the function name
            func_name = node.child_by_field_name('function')
            if func_name:
                c_func_calls.append(func_name.text.decode('utf-8'))
                
            # Check arguments for function pointers
            args = node.child_by_field_name('arguments')
            if args:
                for arg in args.children:
                    # 处理参数节点
                    arg_stack = [arg]
                    while arg_stack:
                        current = arg_stack.pop()
                        if current.type == 'identifier':
                            identifier = current.text.decode('utf-8')
                            if identifier not in c_func_calls:
                                func_as_params.append(identifier)
                        # 将子节点加入栈中继续处理
                        arg_stack.extend(current.children)
        
        # Recursively traverse child nodes
        for child in node.children:
            traverse_node(child)

    traverse_node(root_node)

    return list(dict.fromkeys(c_func_calls + func_as_params))

# 外部调用
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

def init_metadata(metadata: dict, relative_path: str):
    metadata[relative_path] = {}
    metadata[relative_path]['func_signatures'] = []
    metadata[relative_path]['private_func_signatures'] = []
    metadata[relative_path]['public_func_signatures'] = []
    metadata[relative_path]['includes'] = []
    metadata[relative_path]['head_info'] = []
    metadata[relative_path]['variables'] = []
    metadata[relative_path]['functions'] = []
    metadata[relative_path]['rust_items'] = ''

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
        
        code = remove_file_comments(code)
        code = process_alloc_testing(code)

        tree = c_parser.parse(bytes(code, 'utf-8'))
        for node in tree.root_node.children:
            if node.type == 'comment':
                continue
            if node.type == 'preproc_include' and os.path.splitext(os.path.basename(relative_path))[0] not in node.text.decode('utf-8').strip():
                metadata[relative_path]['includes'].append({'code': node.text.decode('utf-8').strip()})
            elif node.type == 'function_definition':
                func_signature = ' '.join(node.text.decode('utf-8').split('{')[0].split()).strip()
                func_signature = func_signature.replace('static ', '')
                func_name = func_signature.split('(')[0].split()[-1].replace('*', '')

                metadata[relative_path]['functions'].append({'name': func_name,'signature': func_signature, 'code': node.text.decode('utf-8'), 'rust_code': '', 'rust_signature': ''})
                metadata[relative_path]['func_signatures'].append(func_signature)
                if func_signature not in metadata[relative_path]['public_func_signatures']:
                    metadata[relative_path]['private_func_signatures'].append(func_signature)
            else:
                if node.text.decode('utf-8') != ';' and node.type != 'preproc_include':
                    if not is_function_declaration(node):
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
        code = remove_file_comments(code)

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
                    # code = code + remove_alloc_test_blocks(func_code) + '\n\n'
                    code += func_code + '\n\n'
            else:
                if not func_code.startswith('static UnitTestFunction tests'):
                    code =code + func_code + '\n\n'
        code = code.strip()


        tree = c_parser.parse(bytes(code, 'utf-8'))
        for node in tree.root_node.children:
            if node.type == 'comment':
                continue
            if node.type == 'preproc_include' and os.path.splitext(os.path.basename(relative_path))[0] not in node.text.decode('utf-8').strip():
                metadata[relative_path]['includes'].append({'code': node.text.decode('utf-8').strip()})
            elif node.type == 'function_definition':
                func_signature = ' '.join(node.text.decode('utf-8').split('{')[0].split()).strip()
                func_signature = func_signature.replace('static ', '')
                func_name = func_signature.split('(')[0].split()[-1].replace('*', '')

                metadata[relative_path]['functions'].append({'name': func_name,'signature': func_signature, 'code': node.text.decode('utf-8'), 'rust_code': '', 'rust_signature': ''})
                metadata[relative_path]['func_signatures'].append(func_signature)
            else:
                if node.text.decode('utf-8') != ';' and node.type != 'preproc_include':
                    metadata[relative_path]['variables'].append({'code': node.text.decode('utf-8')})
    
    # 增加函数依赖
    funcs_files = {}
    for file, file_info in metadata.items():
        for func_info in file_info['functions']:
            funcs_files[file] = func_info['name']

    for file, file_info in metadata.items():
        for func_info in file_info['functions']:
            if 'depend_funcs' not in func_info:
                func_info['depend_funcs'] = []
            func_code = func_info['code']
            func_calls = extract_func_calls(func_code)
            for func_call in func_calls:
                include_file_names = []
                for include in metadata[file]['includes']:
                    include_code = include['code']
                    include_code = include_code.replace('#include', '').strip()
                    include_code = include_code.strip('"<>').replace('.h', '')
                    include_file_names.append(include_code)

                funcs_files = {}
                for file1, file_info1 in metadata.items():
                    file_name = os.path.splitext(os.path.basename(file1))[0]
                    if file_name in include_file_names or file1 == file:
                        for func_info1 in file_info1['functions']:
                            funcs_files[func_info1['name']] = file1

                if func_call in funcs_files and func_call != func_info['name']:
                    func_info['depend_funcs'].append({'name': func_call, 'file': funcs_files[func_call]})

    for func_info in metadata[os.path.join('src','rb-tree.c')]['functions']:
        if func_info['name'] == 'rb_tree_insert_case3':
            for depend_func in func_info['depend_funcs']:
                if depend_func['name'] == 'rb_tree_insert_case1':
                    func_info['depend_funcs'].remove(depend_func)
                    break
            break

    with open('tool/c_metadata_all.json', 'w', encoding='utf-8') as f:
        json.dump(metadata, f, ensure_ascii=False, indent=2)

# if __name__ == "__main__":
#     parser = argparse.ArgumentParser(description='Preprocess C code.')
#     parser.add_argument('--directory', default='./Input/01-Primary', type=str, help='Directory path to preprocess')
#     args = parser.parse_args()

#     code_preprocess(args.directory)