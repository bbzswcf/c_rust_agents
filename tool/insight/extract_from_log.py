import os
import json

from tree_sitter_c_config import rust_parser

# 从c_metadata_direct_output.json中遍历每一个fileinfo，遍历每一个file中的func_info
# 拿到rust_code,再根据file_name和func_info[name]拿到对应文件的标准rust答案
def extract_rust_code_from_log(c_metadata, rust_answers_dir):
    """
    Extract Rust code from log files and match with standard answers.
    
    Args:
        c_metadata_json: Dict containing C metadata and generated Rust code
        rust_answers_dir: Directory containing standard Rust answer files
    """
    results = {}
    
    # Iterate through each file's info
    for file_path, file_info in c_metadata.items():
        if 'test-cpp' in file_path:
            continue
        file_results = []
        file_name = os.path.splitext(os.path.basename(file_path.replace('-', '_')))[0]
        sub_file_name = file_name.replace('test_', '')
        
        # Iterate through each function in the file
        for func_info in file_info['functions']:
            func_name = func_info['name']
            rust_code = func_info['rust_code']
            
            # Get standard answer path
            answer_file = os.path.join(rust_answers_dir, sub_file_name+'.rs')
            
            try:
                with open(answer_file, 'r') as f:
                    standard_answer_code = bytes(f.read(), 'utf-8')
            except FileNotFoundError:
                standard_answer_code = b''
                
            tree = rust_parser.parse(standard_answer_code)

            def find_function_in_tree(node, source_code, func_name):
                if node.type == 'function_item':
                    # Get function name node
                    for child in node.children:
                        if child.type == 'identifier':
                            fn_name = source_code[child.start_byte:child.end_byte].decode('utf-8')
                            if fn_name == func_name:
                                # Return the full function code
                                return source_code[node.start_byte:node.end_byte].decode('utf-8')
                
                # Recursively search children
                for child in node.children:
                    result = find_function_in_tree(child, source_code, func_name)
                    if result:
                        return result
                        
                return None
                
            # Find matching function in standard answer
            standard_answer = find_function_in_tree(tree.root_node, standard_answer_code, func_name)
            if standard_answer:
                file_results.append({
                    'function_name': func_name,
                    'generated_code': rust_code,
                    'standard_answer': standard_answer
                })
            
        results[file_path] = file_results
        
    return results

if __name__ == "__main__":
    c_metadata = json.load(open('tool/c_metadata_direct_output.json', 'r', encoding='utf-8'))
    rust_answers_dir = 'rust-translation/01-Primary/src'
    results = extract_rust_code_from_log(c_metadata, rust_answers_dir)
    # Write results to file
    with open('tool/rust_code_comparison.txt', 'w', encoding='utf-8') as f:
        for file_path, file_results in results.items():
            f.write(f"文件名: {file_path}\n")
            for result in file_results:
                f.write(f"\n函数名: {result['function_name']}\n")
                f.write("生成代码:\n")
                f.write(f"{result['generated_code']}\n")
                f.write("标准答案:\n") 
                f.write(f"{result['standard_answer']}\n")
                f.write("-" * 80 + "\n")