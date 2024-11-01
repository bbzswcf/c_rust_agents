import os
import re
import argparse
import json

from tree_sitter_c_config import c_parser

EXCEPT_FILES = ['framework', 'alloc-testing', 'test-alloc-testing']

def remove_copyright(code: str) -> str:
    """
    Remove copyright notices from C code.
    """
    copyright_pattern = r'^/\*(?:(?!\*/).)*Copyright(?:(?!\*/).)*\*/'
    code = re.sub(copyright_pattern, '', code, flags=re.DOTALL | re.MULTILINE)
    return code.lstrip()

def traverse_node(node, metadata):
    """Extract metadata from a single AST node."""
    if node.type == "preproc_include":
        metadata["includes"].append({
            "text": node.text.decode('utf-8').strip(),
            "start_point": node.start_point,
            "end_point": node.end_point
        })
    elif node.type == "preproc_def":
        if node.child_by_field_name('parameters'):
            metadata["macro_functions"].append({
                "text": node.text.decode('utf-8').strip(),
                "start_point": node.start_point,
                "end_point": node.end_point
            })
        else:
            metadata["macros"].append({
                "text": node.text.decode('utf-8').strip(),
                "start_point": node.start_point,
                "end_point": node.end_point
            })
    elif node.type == "type_definition":
        metadata["types"].append({
            "text": node.text.decode('utf-8').strip(),
            "start_point": node.start_point,
            "end_point": node.end_point
        })
    elif node.type == "declaration":
        metadata["definitions"].append({
            "text": node.text.decode('utf-8').strip(),
            "start_point": node.start_point,
            "end_point": node.end_point
        })
    elif node.type == "function_definition":
        identifier = node.child_by_field_name('declarator')
        if identifier:
            while identifier.type == "pointer_declarator" or identifier.type == "function_declarator":
                identifier = identifier.child_by_field_name('declarator')
            if identifier.type == "identifier":
                metadata["functions"].append({
                    "text": node.text.decode('utf-8').strip(),
                    "identifier": identifier.text.decode('utf-8'),
                    "start_point": node.start_point,
                    "end_point": node.end_point
                })

def traverse_tree(node, metadata):
    traverse_node(node, metadata)
    child_index = 0
    while child_index < len(node.children):
        traverse_tree(node.children[child_index], metadata)
        child_index += 1

def extract_metadata(tree):
    """Extract metadata from a C source file using tree-sitter."""
    metadata = {
        "includes": [],
        "macros": [],
        "macro_functions": [], 
        "types": [],
        "definitions": [],
        "functions": []
    }

    traverse_tree(tree.root_node, metadata)
    return metadata

def analyze_test_files(directory, output_file):
    """Analyze all C files in the test directory and extract metadata."""
    metadata_dict = {}

    # Walk through test directory
    test_dir = os.path.join(directory, 'test')
    if not os.path.exists(test_dir):
        print(f"Test directory not found in {directory}")
        return metadata_dict

    for root, _, files in os.walk(test_dir):
        for file in files:
            if os.path.splitext(file)[0] in EXCEPT_FILES:
                continue
            if file.endswith('.c') or file.endswith('.cpp'):
                file_path = os.path.join(root, file)
                relative_path = os.path.relpath(file_path, directory)
                
                with open(file_path, 'r', encoding='utf-8') as f:
                    
                    tree = c_parser.parse(bytes(remove_copyright(f.read()), 'utf-8'))
                    metadata_dict[relative_path] = extract_metadata(tree)

    # Save to JSON file
    with open(output_file, 'w', encoding='utf-8') as f:
        json.dump(metadata_dict, f, indent=2, ensure_ascii=False)

    return metadata_dict

if __name__ == "__main__":
    parser = argparse.ArgumentParser(description='Extract metadata from C test files')
    parser.add_argument('--directory', default='./Input/01-Primary', help='Directory containing test files')
    parser.add_argument('--output', default='tool/test_metadata.json', help='Output file name')
    args = parser.parse_args()
    
    metadata = analyze_test_files(args.directory, args.output)
    print(f"Metadata extracted and saved to test_metadata.json")
