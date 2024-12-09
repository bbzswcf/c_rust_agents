import os
from tree_sitter_c_config import rust_parser

# 提取extern C声明部分的代码
# 传入文件路径，返回 str
def extract_extern_declaration(path:str):
    if not os.path.exists(path):
        return None
    with open (path) as file:
        code = file.read()
    if code.strip() == '':
        return None
    
    parser = rust_parser
    tree = parser.parse(bytes(code, 'utf8'))
    # 查找extern C声明部分的代码
    def traverse(node):
        if node.type == 'foreign_mod_item':
            start_byte = node.start_byte
            end_byte = node.end_byte
            return code[start_byte:end_byte]
        for child in node.children:
            res = traverse(child)
            if res:
                return res
    result = traverse(tree.root_node) 
    return result if result else ""

# 从extern C的声明块中逐个提取函数签名
# 传入code，返回dict（函数名到签名代码的映射）
def extract_func_signature_from_extern_declaration(code:str):
    if code.strip() == '':
        return None
    
    # 加载parser
    parser = rust_parser
    # 解析代码
    tree = parser.parse(bytes(code, 'utf8'))
    results = {}
    def traverse(node):
        if node.type == 'function_signature_item':
            start_byte = node.start_byte
            end_byte = node.end_byte
            sig_code = code[start_byte:end_byte]

            # 获取函数名
            for child in node.children:
                if child.type == 'identifier':
                    function_name = code[child.start_byte:child.end_byte]
                    results[function_name] = sig_code
                    break
        for child in node.children:
            traverse(child)

    traverse(tree.root_node) 
    return results
        
# 提取全局结构体定义
# 传入文件路径，返回 str[]
def extract_struct_declaration(path:str):
    if not os.path.exists(path):
        return None
    # 解析代码
    with open (path) as file:
        code = file.read()
    if code.strip() == '':
        return None
    
    # 加载parser
    parser = rust_parser
    tree = parser.parse(bytes(code, 'utf8'))
    results = []

    def traverse(node):
        if node.type == 'struct_item':
            start_byte = node.start_byte
            end_byte = node.end_byte
            results.append(code[start_byte:end_byte])
        for child in node.children:
            traverse(child)
    traverse(tree.root_node) 
    return results

# 提取静态变量定义
# 传入文件路径，返回 str[]
def extract_static_declaration(path:str):
    if not os.path.exists(path):
        return None
    # 解析代码
    with open (path) as file:
        code = file.read()
    if code.strip() == '':
        return None
    
    # 加载parser
    parser = rust_parser
    tree = parser.parse(bytes(code, 'utf8'))
    results = []
    def traverse(node):
        if node.type == 'static_item':
            start_byte = node.start_byte
            end_byte = node.end_byte
            cur_decl = code[start_byte:end_byte]
            # 过滤测试函数的static定义
            # if "UnitTestFunction" not in cur_decl:
            results.append(code[start_byte:end_byte])
        for child in node.children:
            traverse(child)
    traverse(tree.root_node) 
    return results

# 提取静态类型定义
# 传入文件路径，返回 str[]
def extract_type_declaration(path:str):
    if not os.path.exists(path):
        return None
    with open (path) as file:
        code = file.read()
    if code.strip() == '':
        return None
    
    # 加载parser
    parser = rust_parser
    tree = parser.parse(bytes(code, 'utf8'))
    results = []

    def traverse(node):
        if node.type == 'type_item':
            start_byte = node.start_byte
            end_byte = node.end_byte
            results.append(code[start_byte:end_byte])
        for child in node.children:
            traverse(child)
    traverse(tree.root_node) 
    return results

# 提取所有函数名
# 传入文件路径，返回 str[]
def extract_function_names(path:str):
    if not os.path.exists(path):
        return None
    with open (path) as file:
        code = file.read()
    if code.strip() == '':
        return []
    
    # 加载parser
    parser = rust_parser
    tree = parser.parse(bytes(code, 'utf8'))
    results = []
    def traverse(node):
        if node.type == 'function_item':
            # 获取函数名
            for child in node.children:
                if child.type == 'identifier':
                    function_name = code[child.start_byte:child.end_byte]
                    results.append(function_name)
                    break
        for child in node.children:
            traverse(child)
    traverse(tree.root_node) 
    return results



if __name__ == "__main__":
    # path = "/mnt/sda/xc/C2Rust/c_rust_agents/vivo-c2rust/output/primary/tests/test_arraylist.rs"
    # with open (path) as file:
    #     code = file.read()

    # extern_code = extract_extern_declaration(path)
    # print(extern_code)
    # results = extract_func_signature_from_extern_declaration(extern_code)
    # for key in results.keys():
    #     print(key)
    #     print(results[key])
    #     if results[key] in code:
    #         print("find")

    # results = extract_struct_declaration(path)
    # for res in results:
    #     print(res)
    #     if res in code:
    #         print("find")

    # results = extract_static_declaration(path)
    # # print(results)
    # for res in results:
    #     print(res)
    #     if res in code:
    #         print("find")



    # results = extract_type_declaration(path)
    # # print(results)
    # for res in results:
    #     print(res)
    #     if res in code:
    #         print("find")
    

    # path = "/mnt/sda/xc/C2Rust/c_rust_agents/vivo-c2rust/cal-safe-ratio/primary/src/arraylist.rs"
    # with open (path) as file:
    #     code = file.read()
    # results = extract_function_names(code)
    # for res in results:
    #     print(res)
    #     if res in code:
    #         print("find")
    

    # file_name = "test_avl_tree.rs"
    # start_index = len("test_")
    # end_index = file_name.find(".rs")
    # module_name = file_name[start_index:end_index]
    # print(module_name)

    results = extract_extern_declaration('./test.rs')
    print(results)

# todo：删除结构体定义和这俩#[derive(Copy, Clone)] #[repr(C)]，删除extern函数定义，提取static item/extern C补进prompt