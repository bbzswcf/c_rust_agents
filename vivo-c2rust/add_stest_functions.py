# 写入lib.rs文件
# test.rs文件中写入use
# 测试文件中重新封装test函数
import json
import os
# 文件路径
metadata_file_path = '../tool/c_metadata.json'
PROJ_DIR = "primary"
TEST_DIR = os.path.join(PROJ_DIR, "tests")
SRC_DIR = os.path.join(PROJ_DIR, "src")
LIB_FILE = os.path.join(SRC_DIR, "lib.rs")


# 打开文件并读取内容
with open(metadata_file_path, 'r') as file:
    metadata = json.load(file)

test_func="""
#[test]
fn {func_name}() {{
    unsafe {{
        {content}
    }}
}}
"""

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
        print(filename)
        
        with open(path, 'a') as file:
            for f in func_names:
                content = f"{f}();"
                file.write("\n")
                file.write(test_func.format(func_name=f"s_{f}", content = content))
                
    # break




