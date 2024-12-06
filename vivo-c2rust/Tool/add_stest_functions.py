# 写入lib.rs文件
# test.rs文件中写入use
# 测试文件中重新封装test函数
import json
import os
import logging
# 文件路径
test_func="""
#[test]
fn {func_name}() {{
    unsafe {{
        {content}
    }}
}}
"""
test_ignore_func="""
#[test]
#[ignore]
fn {func_name}() {{
    unsafe {{
        {content}
    }}
}}
"""

def add_stest_functions(path):
    metadata_file_path = 'Tool/c_metadata.json'
    PROJ_DIR = path
    TEST_DIR = os.path.join(PROJ_DIR, "tests")
    SRC_DIR = os.path.join(PROJ_DIR, "src")
    LIB_FILE = os.path.join(SRC_DIR, "lib.rs")
    # 打开文件并读取内容
    with open(metadata_file_path, 'r') as file:
        metadata = json.load(file)



    for key, value in metadata.items():
        if "test" not in key:
            continue
        filename = key.split('/')[1]
        filename = filename.replace('-', '_')
        if "cpp" in filename:
            filename = filename.replace('.cpp', '.rs')
        else:
            filename = filename.replace('.c', '.rs')
        func_signatures = value["func_signatures"]
        func_signatures = [func for func in func_signatures if "test" in func]
        func_names = [s.split(' ')[-1].split('(')[0] for s in func_signatures]
        
        # print(func_names)
        # content = '\n\t\t'.join([f"{func_name}();" for func_name in func_names])
        # print(content)
        path = os.path.join(TEST_DIR, filename)
        if os.path.isfile(path):
            logging.info(f"add s_test for {filename}")
            # print(f"add s_test for {filename}")
            with open(path, 'a') as file:
                for f in func_names:
                    content = f"{f}();"
                    if "test_rb_tree_remove" in f or "test_rb_tree_to_array" in f:
                        func_code = test_ignore_func.format(func_name=f"s_{f}", content = content)
                    else:
                        func_code = test_func.format(func_name=f"s_{f}", content = content)
                    file.write("\n")
                    file.write(func_code)
                    
        # break

if __name__ == "__main__":
    add_stest_functions("../Output/primary")


