# 写入lib.rs文件
# test.rs文件中写入use
# 测试文件中重新封装test函数
import json
import os
import logging


def add_crate_and_lib(path):
    PROJ_DIR = path
    TEST_DIR = os.path.join(PROJ_DIR, "tests")
    SRC_DIR = os.path.join(PROJ_DIR, "src")
    LIB_FILE = os.path.join(SRC_DIR, "lib.rs")
    # 文件路径
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
    logging.info("lib.rs 文件已更新。")



    for root, dirs, files in os.walk(TEST_DIR):
        for file in files:
            logging.info(f"add import for {file}")
            path = os.path.join(root, file)
            with open(path, 'r') as f:
                lines = f.readlines()
            for i, line in enumerate(lines):
                if line.strip() == '' or line.startswith("#"):
                    continue
                lines.insert(i, "use primary::hash_int::*;\n")
                lines.insert(i, "use primary::hash_string::*;\n")
                lines.insert(i, "use primary::hash_pointer::*;\n")
                lines.insert(i, "use primary::compare_int::*;\n")
                lines.insert(i, "use primary::compare_string::*;\n")
                lines.insert(i, "use primary::compare_pointer::*;\n")
                if "compare_functions" not in file and "hash_functions" not in file and "cpp" not in file:
                    lines.insert(i, f"use primary::{file.removesuffix('.rs').removeprefix('test_')}::*;\n")
                lines.insert(i, "use primary::alloc_testing::*;\n")
                lines.insert(i, "extern crate libc;\n")
                break
            with open(path, 'w') as f:
                f.writelines(lines)

    ignore=["compare_int.rs","compare_string.rs","compare_pointer.rs",
        "hash_int.rs","hash_string.rs","hash_pointer.rs"]
    for root, dirs, files in os.walk(SRC_DIR):
        for file in files:
            logging.info(f"add import for {file}")
            # print(f"add import for {file}")
            if "alloc_testing.rs" in file or "lib.rs" in file:
                continue
            if file in ignore:
                continue
            path = os.path.join(root, file)
            with open(path, 'r') as f:
                lines = f.readlines()
            for i, line in enumerate(lines):
                if line.strip() == '' or line.startswith("#"):
                    continue
                lines.insert(i, "use crate::alloc_testing::*;\n")
                lines.insert(i, "use std::mem;\n")
                break
            with open(path, 'w') as f:
                f.writelines(lines)




if __name__ == "__main__":
    add_crate_and_lib("../output/primary")