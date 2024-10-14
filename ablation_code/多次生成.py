import os
import re
import shutil
import subprocess
import sys
import chardet
from openai import OpenAI
import stat
# 设置默认编码为UTF-8
sys.stdout.reconfigure(encoding='utf-8')
sys.stderr.reconfigure(encoding='utf-8')

# OpenAI 客户端设置
# deepseek_key = "sk-af84595199ce478fb7c4e6065ef6d901"
# deepseek_base_url = "https://api.deepseek.com/beta"
# client = OpenAI(api_key=deepseek_key, base_url=deepseek_base_url)
siliconflow_key = "sk-syxychvjsafogqdkspiegsrhovomwzoapmvokcrdeperzadv"
siliconflow_base_url = "https://api.siliconflow.cn/v1"
client = OpenAI(api_key=siliconflow_key, base_url=siliconflow_base_url)
# 目录设置
c_code_dir = "test_多次输出_人工审核\c_codes"
c_out_dir = "test_多次输出_人工审核\c_outputs"
rust_code_dir = "test_多次输出_人工审核\Translate_Rust_codes"
rust_out_dir = "test_多次输出_人工审核\Translate_Rust_outputs"


def normalize_string(s):
    '''忽略大小写，去除所有空格，格式化浮点数并去除多余0.
    '''
    if isinstance(s, subprocess.CompletedProcess):
        s = s.stdout  # 使用 CompletedProcess 的 stdout 属性

    if not isinstance(s, str):
        return str(s)  # 如果不是字符串，转换为字符串

    s = s.lower()
    s = re.sub(r'\s+', '', s)
    if not re.search(r'\d', s):
        return s
    try:
        float_pattern = re.compile(r'[-+]?\d*\.\d+([eE][-+]?\d+)?')
        matches = list(float_pattern.finditer(s))
        for match in reversed(matches):
            original_float = match.group()
            formatted_number = f"{float(original_float)}"
            s = s[:match.start()] + formatted_number + s[match.end():]
    except Exception as e:
        print("浮点数格式化发生错误，已返回原字符串")

    return s
def detect_encoding(file_path):
    with open(file_path, 'rb') as file:
        raw_data = file.read()
    return chardet.detect(raw_data)['encoding']

def read_file_with_auto_encoding(file_path):
    encoding = detect_encoding(file_path)
    with open(file_path, 'r', encoding=encoding, errors='replace') as file:
        return file.read()

def write_file_with_utf8(file_path, content):
    with open(file_path, 'w', encoding='utf-8', errors='ignore') as file:
        file.write(content)

def simple_prompt(input_content, temp_prm, top_p_prm):
    input_content = f"""\

Translate the C code to Rust. Output only the converted Rust code without explanations.
The code should be idiomatic Rust, do not use libc or ffi!:
    c
    {input_content}
    """
    succeed = False
    while not succeed:
        try:
            response = client.chat.completions.create(
                model="deepseek-ai/DeepSeek-V2.5",
                messages=[{"role": "user", "content": input_content}],
                temperature=temp_prm,
                top_p=top_p_prm,
                max_tokens=2048,
                stream=True,
            )
            succeed = True
        except Exception as e:
            print(f"API 调用失败，正在重试: {e}")

    full_response = ""
    for chunk in response:
        if chunk.choices[0].delta.content is not None:
            print(chunk.choices[0].delta.content, end='', flush=True)
            full_response += chunk.choices[0].delta.content
    print()  # 换行

    rust_codes = re.findall(r'```rust\n?(.*?)\n?```', full_response, re.DOTALL)
    return rust_codes[0] if rust_codes else full_response

def compile_and_run_rust(rust_code: str, problem_folder: str, c_output_file: str):
    current_dir = os.getcwd()
    file_path = os.path.join(current_dir, "temp_main.rs")
    exe_path = os.path.join(current_dir, "temp_main.exe" if sys.platform == "win32" else "temp_main")
    write_file_with_utf8(file_path, rust_code)
    try:
        # 编译
        result = subprocess.run(["rustc", file_path, "-o", exe_path], capture_output=True, text=True)
        if result.returncode != 0:
            return None, None, False, f"编译错误: {result.stderr}"

        # 检查可执行文件是否存在
        if not os.path.exists(exe_path):
            return None, None, False, f"未在 {exe_path} 找到可执行文件"

        # 运行
        try:
            output = subprocess.run([exe_path], capture_output=True, text=True, timeout=5)
        except subprocess.TimeoutExpired:
            return None, None, True, "执行超时"
        except FileNotFoundError:
            return None, None, False, f"无法运行 {exe_path}"

        if output.returncode != 0:
            return None, None, True, f"运行时错误: {output.stderr}"

        # 读取 C 程序的输出
        c_output = read_file_with_auto_encoding(c_output_file)

        # 比较输出
        normalized_rust_output = normalize_string(output.stdout)
        normalized_c_output = normalize_string(c_output)
        if normalized_rust_output == normalized_c_output:
            return output.stdout, True, True, None
        else:
            return output.stdout, False, True, None

    finally:
        if os.path.exists(file_path):
            os.remove(file_path)
        if os.path.exists(exe_path):
            os.chmod(exe_path, stat.S_IWRITE)  # 修改文件权限
            os.remove(exe_path)

def convert_c_to_rust(c_code: str, problem_folder: str, c_output_file: str):
    temperatures = [0.01, 0.4, 0.4, 0.8, 0.8]
    top_ps = [0.9, 0.9, 0.9, 0.9, 0.9]
    best_code = None
    best_output_length = -1
    output_matches = False
    compiles = False
    longest_code = ""
    compile_error = None

    for temp, top_p in zip(temperatures, top_ps):
        print(f"尝试temperature {temp} 和 top_p {top_p}")
        rust_code = simple_prompt(c_code, temp, top_p)

        # 更新最长代码
        if len(rust_code) > len(longest_code):
            longest_code = rust_code

        result = compile_and_run_rust(rust_code, problem_folder, c_output_file)
        output_length, matches, does_compile, error = result

        if does_compile:
            compiles = True
            if matches and (best_code is None or (output_length is not None and output_length > best_output_length)):
                best_code = rust_code
                best_output_length = output_length if output_length is not None else -1
                output_matches = True
                print(f"找到匹配的版本，输出长度: {output_length}")
            elif not output_matches and (best_code is None or (output_length is not None and output_length > best_output_length)):
                best_code = rust_code
                best_output_length = output_length if output_length is not None else -1
        else:
            compile_error = error

    if best_code is None:
        best_code = longest_code

    return best_code, output_matches, compiles, compile_error
def process_files():
    succeed_rust_dir = os.path.join(rust_code_dir, "succeed")
    succeed_out_dir = os.path.join(rust_out_dir, "succeed")
    mismatch_rust_dir = os.path.join(rust_code_dir, "mismatch")
    mismatch_out_dir = os.path.join(rust_out_dir, "mismatch")
    not_compile_dir = os.path.join(rust_code_dir, "not_compile")

    for dir_path in [succeed_rust_dir, succeed_out_dir, mismatch_rust_dir, mismatch_out_dir, not_compile_dir]:
        os.makedirs(dir_path, exist_ok=True)

    successful_conversions = 0
    mismatch_conversions = 0
    compile_failures = 0
    total_files = 0
    error_log = []

    for problem_folder in os.listdir(c_code_dir):
        problem_path = os.path.join(c_code_dir, problem_folder)
        if not os.path.isdir(problem_path):
            continue

        total_files += 1
        c_file_path = os.path.join(problem_path, "main.c")
        c_out_file_path = os.path.join(c_out_dir, f"{problem_folder}.out")

        if not os.path.exists(c_file_path):
            print(f"C 文件不存在: {c_file_path}")
            error_log.append(f"C 文件不存在: {c_file_path}")
            continue

        try:
            c_code = read_file_with_auto_encoding(c_file_path)
        except Exception as e:
            print(f"读取 C 文件 {c_file_path} 时发生错误: {str(e)}")
            error_log.append(f"读取 C 文件 {c_file_path} 时发生错误: {str(e)}")
            continue

        print(f"开始转换 {problem_folder}")

        try:
            rust_code, output_matches, compiles, compile_error = convert_c_to_rust(c_code, problem_folder, c_out_file_path)
        except Exception as e:
            print(f"转换 {problem_folder} 时发生错误: {str(e)}")
            error_log.append(f"转换 {problem_folder} 时发生错误: {str(e)}")
            continue

        if compiles:
            if output_matches:
                successful_conversions += 1
                print(f"成功转换 {problem_folder}，输出匹配")
                write_file_with_utf8(os.path.join(succeed_rust_dir, f"{problem_folder}.rs"), rust_code)
                try:
                    shutil.copy(c_out_file_path, os.path.join(succeed_out_dir, f"{problem_folder}.out"))
                except Exception as e:
                    print(f"复制输出文件时发生错误: {str(e)}")
                    error_log.append(f"复制输出文件时发生错误: {str(e)}")
            else:
                mismatch_conversions += 1
                print(f"转换 {problem_folder} 成功，但输出不匹配")
                write_file_with_utf8(os.path.join(mismatch_rust_dir, f"{problem_folder}.rs"), rust_code)
                try:
                    rust_output, _, _, _ = compile_and_run_rust(rust_code, problem_folder, c_out_file_path)
                    if rust_output is not None:
                        write_file_with_utf8(os.path.join(mismatch_out_dir, f"{problem_folder}_rust.out"), rust_output)
                except Exception as e:
                    print(f"运行不匹配的 Rust 代码时发生错误: {str(e)}")
                    error_log.append(f"运行不匹配的 Rust 代码时发生错误: {str(e)}")

        else:
            compile_failures += 1
            print(f"转换 {problem_folder} 失败，保存最长的生成代码和编译错误信息")
            write_file_with_utf8(os.path.join(not_compile_dir, f"{problem_folder}.rs"), rust_code)
            write_file_with_utf8(os.path.join(not_compile_dir, f"{problem_folder}_error.txt"), compile_error)

    print("\n转换统计:")
    print(f"总文件数: {total_files}")
    print(f"成功转换且输出匹配数: {successful_conversions}")
    print(f"成功转换但输出不匹配数: {mismatch_conversions}")
    print(f"编译失败数: {compile_failures}")
    print(f"转换成功率: {successful_conversions / total_files:.2%}")

    if error_log:
        print("\n错误日志:")
        for error in error_log:
            print(error)

    # 将错误日志写入文件
    with open("error_log.txt", "w", encoding="utf-8") as f:
        for error in error_log:
            f.write(f"{error}\n")

if __name__ == "__main__":
    process_files()