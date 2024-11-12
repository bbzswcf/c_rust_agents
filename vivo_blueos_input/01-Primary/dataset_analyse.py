# 统计最大最小平均行数
# 使用：改TARGET_DIRECTORY路径
import os
import subprocess
import re
TARGET_DIRECTORY = "./no_comments"

def count_lines_in_directory(directory):
    total_lines = 0
    file_count = 0
    max_lines = 0
    min_lines = float('inf')  # 初始化为一个很大的值

    # 遍历目录下的所有文件
    for root, dirs, files in os.walk(directory):
        for file in files:
            file_path = os.path.join(root, file)
            try:
                with open(file_path, 'r', encoding='utf-8') as f:
                    lines = f.readlines()
                    line_count = len(lines)

                    # 更新总行数和文件计数
                    total_lines += line_count
                    file_count += 1

                    if (line_count == 16):
                        print(file)
                    # 更新最大行数和最小行数
                    if line_count > max_lines:
                        max_lines = line_count
                    if line_count < min_lines:
                        min_lines = line_count
            except Exception as e:
                print(f"无法读取文件 {file_path}: {e}")

    # 计算平均行数
    if file_count > 0:
        average_lines = total_lines / file_count
    else:
        average_lines = 0

    # 输出结果
    print(f"总行数: {total_lines}")
    print(f"文件数: {file_count}")
    print(f"最大行数: {max_lines}")
    print(f"最小行数: {min_lines}")
    print(f"平均行数: {average_lines:.2f}")



# 调用函数统计行数
count_lines_in_directory(TARGET_DIRECTORY)
