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


# 清空或创建 lib.rs 文件
open(LIB_FILE, 'w').close()
# 遍历指定目录下的所有文件
for filename in os.listdir(SRC_DIR):
    file_path = os.path.join(SRC_DIR, filename)
    # 确保是文件而非目录
    if os.path.isfile(file_path) and filename != "lib.rs":
        # 去除文件后缀，只保留文件名
        modulename = os.path.splitext(filename)[0]
        # 将 pub mod 文件名; 写入 lib.rs 文件
        with open(LIB_FILE, 'a') as lib_file:
            lib_file.write(f"pub mod {modulename};\n")
print("lib.rs 文件已更新。")



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
        with open(path, 'r') as file:
            lines = file.readlines()
        for i, line in enumerate(lines):
            if line.strip() == '' or line.startswith("#"):
                continue
            lines.insert(i, "use primary::hash_int::*;\n")
            lines.insert(i, "use primary::hash_string::*;\n")
            lines.insert(i, "use primary::hash_pointer::*;\n")
            lines.insert(i, "use primary::compare_int::*;\n")
            lines.insert(i, "use primary::compare_string::*;\n")
            lines.insert(i, "use primary::compare_pointer::*;\n")
            if "compare_functions" not in filename and "hash_functions" not in filename and "cpp" not in filename:
                lines.insert(i, f"use primary::{filename.removesuffix('.rs').removeprefix('test_')}::*;\n")
            lines.insert(i, "extern crate libc;\n")
            break
        with open(path, 'w') as file:
            file.writelines(lines)
        with open(path, 'a') as file:
            for f in func_names:
                content = f"{f}();"
                file.write("\n")
                file.write(test_func.format(func_name=f"s_{f}", content = content))
                
    # break




