import os
import shutil

# 定义文件夹路径
not_compile_folder = r'实验数据\test_没有测试输出不匹配\Translate_Rust_Codes\mismatch'
c_code_folder = r'实验数据\test_没有测试输出不匹配\c_codes'
c_not_compile_code_folder = r'test1\c_codes'

# 移动的文件数量计数
moved_count = 0

# 确保目标文件夹存在，不存在则创建
if not os.path.exists(c_not_compile_code_folder):
    os.makedirs(c_not_compile_code_folder)

# 从 not_compile 文件夹中读取所有的 .rs 文件名（不包含后缀）
rs_filenames = [os.path.splitext(file)[0] for file in os.listdir(not_compile_folder) if file.endswith('.rs')]

# 遍历 c_code 文件夹，查找同名的 .c 文件并移动到 c_not_compile_code 文件夹中
for file in os.listdir(c_code_folder):
    # 如果文件是 .c 后缀且文件名在 rs_filenames 中，则移动文件
    if file in rs_filenames:
        source_path = os.path.join(c_code_folder, file)
        destination_path = os.path.join(c_not_compile_code_folder, file)
        shutil.copytree(source_path, destination_path)
        moved_count += 1
        print(f"已移动: {file} 到 {c_not_compile_code_folder}")

# 输出移动的文件总数量
print(f"总共移动了 {moved_count} 个 .c 文件到 {c_not_compile_code_folder}")
