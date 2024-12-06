import os
from tree_sitter import Language, Parser

def function_replace(file_path, origin_code, new_code):
    with open(file_path,'r') as file:
        content = file.read()
    if origin_code not in content:
        return False
    with open(file_path,'w') as file:
        file.write(content.replace(origin_code, new_code))
    return True

def remove_alloc_test_blocks_in_code(code: str) -> str:
# Remove alloc_test blocks from test files, keeping only the closing brace.
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

    # end_idx = code.rfind('}')
    end_idx = code.find('}', start_idx)
    if end_idx == -1:
        return code
    print('-' * 40)
    print(code[:start_idx] + code[end_idx:])
    # Keep everything before alloc_test and after the last brace
    return code[:start_idx] + code[end_idx:]


def split_c_functions_in_file(file_path):
    if os.path.isfile(file_path):
        try:
            # 直接使用 dll 文件路径创建 Language 对象
            dll_path = os.path.abspath(os.path.join('build', 'c.so'))
            language = Language(dll_path, 'c')
            
            parser = Parser()
            parser.set_language(language)
            
        except Exception as e:
            print(f"加载语言时出错: {e}")
            raise e
    
        try:
            with open(file_path, 'r', encoding='utf-8') as f:
                code = f.read()
            # 解析代码
            tree = parser.parse(bytes(code, 'utf8'))
            # 存储当前文件的函数
            functions = []
            # 使用深度优先搜索查找所有函数定义
            def traverse(node):
                if node.type == 'function_definition':
                    start_byte = node.start_byte
                    end_byte = node.end_byte
                    function_code = code[start_byte:end_byte]
                    
                    # 获取函数名
                    for child in node.children:
                        if child.type == 'function_declarator':
                            name_node = child.child_by_field_name("declarator")
                            function_name = code[name_node.start_byte:name_node.end_byte]
                            functions.append({
                                'name': function_name,
                                'code': function_code,
                                'start': start_byte,
                                'end': end_byte
                            })
                            break
                            
                for child in node.children:
                    traverse(child)
            
            traverse(tree.root_node)
            return functions
            
        except Exception as e:
            print(f"处理文件 {file_path} 时发生错误: {str(e)}")
    else:
        print(f"{file_path} 不是文件路径")
        return None


def remove_alloc_test_blocks_in_file(file_path: str):
    functions = split_c_functions_in_file(file_path)
    if functions == None:
        return 
    for func in functions:
        origin_code = func['code']
        new_code = remove_alloc_test_blocks_in_code(origin_code)
        function_replace(file_path, origin_code, new_code)

def remove_alloc_test_blocks_in_dir(dir_path: str):
    for root, dirs, files in os.walk(dir_path):
        for file in files:
            # 这个文件无需处理
            if "alloc-testing" in file:
                continue
            file_path = os.path.join(root, file)
            remove_alloc_test_blocks_in_file(file_path)


if __name__ == "__main__":


    path = "/mnt/sda/xc/C2Rust/c_rust_agents/vivo-c2rust/input/01-Primary/test/test-binomial-heap.c"
    # res = split_c_functions_in_file(path)
    # for func in res:
    #     if func['name'] == "test_arraylist_insert":
    #         print(func['code'])
    #         print("-" * 40)
    #         new_code = remove_alloc_test_blocks_in_code(func['code'])
    #         print(new_code)
    #         print("-" * 40)
    #         function_replace(path, func['code'], new_code)
    remove_alloc_test_blocks_in_file(path)
