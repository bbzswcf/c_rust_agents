"""
This script decomposes C code into blocks based on function boundaries.
"""
import os

from preprocess.c_code_preprocess import preprocess
from tree_sitter_c_config import c_parser

def decompose(c_code: str):
    """
    Decompose C code into blocks based on function boundaries.
    """
    tree = c_parser.parse(bytes(c_code, "utf8"))
    root_node = tree.root_node
    
    function_nodes = []
    for child in root_node.children:
        if child.type == 'function_definition':
            function_nodes.append(child)

    blocks = []
    
    if function_nodes:
        first_func = function_nodes[0]
        prev_sibling = first_func.prev_sibling
        
        if prev_sibling and prev_sibling.type == 'comment':
            blocks.append(c_code[0:prev_sibling.start_byte].strip())
            blocks.append(c_code[prev_sibling.start_byte:first_func.end_byte].strip())
        else:
            blocks.append(c_code[0:first_func.start_byte].strip())
            blocks.append(c_code[first_func.start_byte:first_func.end_byte].strip())
        
        for i in range(len(function_nodes)-1):
            current_func = function_nodes[i]
            next_func = function_nodes[i + 1]
            blocks.append(c_code[current_func.end_byte:next_func.end_byte].strip())
    else:
        blocks.append(c_code.strip())

    return blocks

def code_decomposition(c_code_dir, depend_file, depend_funcs, start_byte):
    """
    Decompose C code into blocks based on function boundaries.
    """
    file_path = None
    for root, dirs, files in os.walk(c_code_dir):
        if depend_file in files:
            file_path = os.path.join(root, depend_file)
            break
    
    file_path = os.path.splitext(file_path)[0] + '.c'
    with open(file_path, 'r', encoding='utf-8') as f:
        code = preprocess(f.read())
    
    tree = c_parser.parse(bytes(code, "utf8"))
    root_node = tree.root_node

    func_locations = {}

    # Find all function definitions
    for node in root_node.children:
        if node.type == 'function_definition':
            # Get the function declarator node which contains the function name
            declarator = None
            for child in node.children:
                if child.type == 'function_declarator':
                    declarator = child
                    break
                elif child.type == 'pointer_declarator':
                    declarator = child.children[1]
                    break
            
            if declarator:
                # Get the function name from the declarator
                for child in declarator.children:
                    if child.type == 'identifier':
                        func_name = child.text.decode('utf-8')
                        if func_name in depend_funcs:
                            # Store the function start and end points
                            func_locations[func_name] = (node.start_byte, node.end_byte)

    # Find the maximum end point among all functions
    max_end_byte = 0
    for _, location in func_locations.items():
        _, end = location
        if end > max_end_byte:
            max_end_byte = end

    # Extract the code from the start to the maximum end byte
    if start_byte >= max_end_byte:
        return [], start_byte
    extracted_code = code[start_byte:max_end_byte]
    blocks = decompose(extracted_code)
    
    return blocks, max_end_byte
    