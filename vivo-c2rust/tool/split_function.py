import os
from tree_sitter import Language, Parser


def split_rust_functions_in_file(file_path):
    if os.path.isfile(file_path):
        try:
            # 直接使用 dll 文件路径创建 Language 对象
            dll_path = os.path.abspath(os.path.join('build', 'rust.so'))
            language = Language(dll_path, 'rust')
            
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
                if node.type == 'function_item':
                    start_byte = node.start_byte
                    end_byte = node.end_byte
                    function_code = code[start_byte:end_byte]
                    
                    # 获取函数名
                    for child in node.children:
                        if child.type == 'identifier':
                            function_name = code[child.start_byte:child.end_byte]
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


def split_rust_functions(folder_path):
    """
    使用tree-sitter解析Rust文件并提取所有函数
    """
    try:
        # 直接使用 dll 文件路径创建 Language 对象
        dll_path = os.path.abspath(os.path.join('build', 'rust.so'))
        language = Language(dll_path, 'rust')
        
        parser = Parser()
        parser.set_language(language)
        
    except Exception as e:
        print(f"加载语言时出错: {e}")
        raise e
    
    # 存储结果的字典
    result = {}
     
    # 遍历文件夹中的所有Rust文件
    for root, dirs, files in os.walk(folder_path):
        for file in files:
            if file.endswith('.rs'):
                file_path = os.path.join(root, file)
                res = split_rust_functions_in_file(file_path=file_path)
                if res:
                    result[file_path] = res
                else:
                    print(f"处理文件{file_path}异常返回")
    return result

# 'name': function_name
# 'code': function_code
# 'start': start_byte
# 'end': end_byte
def print_functions(functions_dict):
    """
    打印提取的函数信息
    
    Args:
        functions_dict: 从split_rust_functions返回的字典
    """
    for file_path, functions in functions_dict.items():
        print(f"\n文件: {file_path}")
        print("=" * 80)
        
        for func in functions:
            print(f"\n函数名: {func['name']}")
            print("-" * 40)
            print(func['code'])
            print("-" * 40)

def save_functions_to_files(functions_dict, output_dir):
    """
    将提取的函数保存到单独的文件中
    
    Args:
        functions_dict: 从split_rust_functions返回的字典
        output_dir: 输出目录的路径
    """
    # 确保输出目录存在
    os.makedirs(output_dir, exist_ok=True)
    
    for file_path, functions in functions_dict.items():
        base_name = os.path.basename(file_path)
        file_name_without_ext = os.path.splitext(base_name)[0]
        
        file_dir = os.path.join(output_dir, file_name_without_ext)
        os.makedirs(file_dir, exist_ok=True)
        
        for func in functions:
            # 清理文件名
            safe_function_name = "".join(c for c in func['name'] if c.isalnum() or c in '_-')
            if not safe_function_name:  # 如果函数名清理后为空，直接跳过
                continue
            
            output_file = os.path.join(file_dir, f"{safe_function_name}.rs")
            
            with open(output_file, 'w', encoding='utf-8') as f:
                f.write(func['code'])

# 修改使用示例:
# result = split_rust_functions(folder_path="/mnt/sda/xc/C2Rust/c_rust_agents/vivo-c2rust/cal-safe-ratio/init/")
# print_functions(result)  # 如果还需要打印的话
# save_functions_to_files(result, "Rust_functions")

if __name__ == "__main__":
    result = split_rust_functions_in_file("/mnt/sda/xc/C2Rust/c_rust_agents/vivo-c2rust/primary/src/avl_tree.rs")
    for func in result:
        print(f"\n函数名: {func['name']}")
        print("-" * 40)
        print(func['code'])
        print("-" * 40)
