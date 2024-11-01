import re
import argparse
import os
import json

from tree_sitter_c_config import c_parser

EXCEPT_FILES = ['framework.h', 'alloc-testing.h']

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

def head_preprocess(code: str) -> str:
    code = remove_file_comments(code)
    code = remove_header_guards(code) 
    code = remove_cpp_guards(code)
    return code

def node_to_dict(node):
    """Convert a tree-sitter node to a dictionary format."""
    result = {
        'type': node.type,
        'start_point': node.start_point,
        'end_point': node.end_point,
        'text': node.text.decode('utf8')
    }
    if len(node.children) > 0:
        result['children'] = [node_to_dict(child) for child in node.children]
    return result

def analyze_headers(directory: str, output_file: str):
    """
    Analyze all header files in a directory and its subdirectories.
    Save the analysis results to a JSON file.
    """
    analysis_results = {}

    # Walk through directory
    for root, _, files in os.walk(directory):
        for file in files:
            if file.endswith('.h'):
                if file in EXCEPT_FILES:
                    continue
                file_path = os.path.join(root, file)
                relative_path = os.path.relpath(file_path, directory)
                
                # Preprocess header file
                with open(file_path, 'r', encoding='utf-8') as f:
                    code = f.read()
                
                preprocessed_code = head_preprocess(code)
                
                # Parse with tree-sitter
                tree = c_parser.parse(bytes(preprocessed_code, 'utf8'))
                
                # Store analysis result
                analysis_results[relative_path] = node_to_dict(tree.root_node)
    
    # Save to JSON file
    with open(output_file, 'w', encoding='utf-8') as f:
        json.dump(analysis_results, f, indent=2, ensure_ascii=False)

if __name__ == "__main__":
    
    parser = argparse.ArgumentParser(description='Analyze C header files in a directory')
    parser.add_argument('--directory', default='./Input/01-Primary', help='Directory containing header files')
    parser.add_argument('--output', default='tool/header_analysis.json', help='Output JSON file path')
    
    args = parser.parse_args()
    analyze_headers(args.directory, args.output)
