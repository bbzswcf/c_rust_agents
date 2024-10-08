import os
import re
import shutil
import tempfile
from Agents import *
import subprocess
import sys
import chardet
import unicodedata
# 设置默认编码为UTF-8
sys.stdout.reconfigure(encoding='utf-8')
sys.stderr.reconfigure(encoding='utf-8')

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

def static_analysis(rust_code: str) -> str:
    with tempfile.TemporaryDirectory() as tmpdir:
        rust_file = os.path.join(tmpdir, "temp.rs")
        write_file_with_utf8(rust_file, rust_code)

        # 使用 clippy 进行静态分析
        clippy_result = subprocess.run(["clippy-driver", rust_file],
                                       capture_output=True, text=True, encoding="utf-8")

        issues = []

        # 解析 clippy 输出，只关注错误
        clippy_output = clippy_result.stdout + clippy_result.stderr
        error_blocks = re.findall(r'error:.*?(?=\nerror:|\Z)', clippy_output, re.DOTALL)
        for block in error_blocks:
            if block.strip():
                issues.append(block.strip())

        # 提取错误代码并获取详细说明
        error_codes = set(re.findall(r'error\[E(\d+)]', clippy_output))
        for code in error_codes:
            explain_result = subprocess.run(["rustc", "--explain", f"E{code}"],
                                            capture_output=True, text=True, encoding="utf-8")
            explanation = explain_result.stdout.strip()
            issues.append(f"错误 E{code} 的详细说明:\n{explanation}")

        # 添加总结性错误信息
        summary_match = re.search(r'error: aborting due to (\d+) previous errors?; (\d+) warnings? emitted', clippy_output)
        if summary_match:
            error_count, warning_count = summary_match.groups()
            issues.append(f"总计: {error_count} 个错误, {warning_count} 个警告")

        return "\n\n".join(issues) if issues else ""

def compile_and_test_rust(rust_code: str, c_output_file: str, rust_output_file: str) -> tuple[bool, str]:
    current_dir = os.getcwd()
    rust_file = os.path.join(current_dir, "temp_main.rs")
    rust_exe = os.path.join(current_dir, "temp_main.exe" if sys.platform == "win32" else "temp_main")

    try:
        write_file_with_utf8(rust_file, rust_code)
        print(f"Rust 代码已写入: {rust_file}")
        # 添加 -A dead_code 和 -A unused_variables 标志来忽略未使用变量的警告
        compile_command = ["rustc", "-A", "dead_code", "-A", "unused_variables", rust_file, "-o", rust_exe]
        compile_result = subprocess.run(compile_command, capture_output=True, text=True, encoding="utf-8")

        if compile_result.returncode != 0:
            return False, f"Rust 编译失败:\n{compile_result.stderr}"

        if not os.path.exists(rust_exe):
            return False, f"错误：找不到编译后的可执行文件: {rust_exe}"

        try:
            run_result = subprocess.run([rust_exe], capture_output=True, text=True, encoding="utf-8", timeout=5)
        except subprocess.TimeoutExpired:
            return False, "Rust 程序运行超时"

        if run_result.returncode != 0:
            return False, f"Rust 程序运行失败:\n{run_result.stderr}"

        rust_output = run_result.stdout
        write_file_with_utf8(rust_output_file, rust_output)

        c_output = read_file_with_auto_encoding(c_output_file)

        if rust_output.strip() == c_output.strip():
            return True, ""
        else:
            return True, f"输出不匹配。\nC 输出:\n{c_output}\nRust Outputs:\n{rust_output}"

    finally:
        if os.path.exists(rust_file):
            os.remove(rust_file)
        if os.path.exists(rust_exe):
            os.remove(rust_exe)

def sanitize_string(s):
    """
    将字符串中的非ASCII字符替换为它们的ASCII表示或删除。
    """
    return ''.join(c for c in unicodedata.normalize('NFKD', s) if ord(c) < 128)

def convert_c_to_rust(c_code: str, c_output_file: str, rust_output_file: str) -> str:
    if c_code is None or c_code.strip() == "":
        print("错误：没有提供有效的 C 代码进行转换")
        return ""

    print("开始API转换")
    api_conversion = api_agent.generate_response(
        f"""
        Extract and convert only the C-specific APIs to their Rust equivalents:
        {c_code}
        """
    )

    print("开始语法转换")
    combined_syntax_input = f"""
    Convert the following C code to Rust using the provided API mappings:
    C code:
    {c_code}
    API mappings:
    {api_conversion}
    Remember to output only the converted Rust code without any explanations.
    """

    rust_code = syntax_agent.generate_response(combined_syntax_input)
    rust_code = extract_rust_code(rust_code)

    max_static_analysis_attempts = 5
    max_compile_attempts = 5
    extra_mismatch_attempts = 3

    for static_attempt in range(1, max_static_analysis_attempts + 1):
        print(f"静态分析尝试 #{static_attempt}")

        analysis_result = static_analysis(rust_code)
        if not analysis_result:
            print("静态分析未发现错误，进入编译和测试阶段。")
            break

        print("发现静态分析问题，正在优化...")
        feedback_input = f"""
        Analyze the following static analysis results:
        Issue description:
        {sanitize_string(analysis_result)}
        Current Rust code:
        {sanitize_string(rust_code)}
        Please provide specific fix suggestions, but do not generate improved code.
        """
        feedback = feedback_agent.generate_response(feedback_input)

        optimize_input = f"""
        Optimize the Rust code based on the following specific feedback:
        Feedback:
        {sanitize_string(feedback)}
        Current Rust code:
        {sanitize_string(rust_code)}
        Ensure that no new bugs are introduced.
        Please strictly follow the steps mentioned in the prompt to optimize the code. Ensure all issues mentioned in the feedback are resolved, and add comments for each modification explaining the reason. 
        Only return the complete optimized Rust code without additional explanations.
        """

        optimized = optimize_agent.generate_response(optimize_input)
        new_rust_code = extract_rust_code(optimized)

        if new_rust_code.strip():
            rust_code = new_rust_code
            print(f"代码已针对静态分析进行优化 #{static_attempt}")
        else:
            print("警告：代码优化专家没有返回有效的Rust代码。保持原代码不变。")

    for compile_attempt in range(1, max_compile_attempts + 1):
        print(f"编译和测试尝试 #{compile_attempt}")
        compile_success, error_or_mismatch_info = compile_and_test_rust(rust_code, c_output_file, rust_output_file)

        if compile_success and not error_or_mismatch_info:
            print("编译和测试成功")
            return rust_code

        if compile_attempt == max_compile_attempts:
            if compile_success:  # 编译成功但输出不匹配
                print("达到最大编译尝试次数,但输出不匹配。进行额外的优化尝试。")
                for extra_attempt in range(1, extra_mismatch_attempts + 1):
                    print(f"额外优化尝试 #{extra_attempt}")
                    mismatch_feedback_input = f"""
                    Analyze the following results:
                    Issue type:Output Mismatch
                    Issue description:
                    {sanitize_string(error_or_mismatch_info)}
                    Current Rust Code：
                    {sanitize_string(rust_code)}
                    Pay attention to details that may cause differences in output, such as formatting, precision, line breaks, etc.
                    """

                    feedback = feedback_agent.generate_response(mismatch_feedback_input)
                    mismatch_optimize_input = f"""
                    Optimize the Rust code to solve the output mismatch problem based on the following specific feedback:
                    Feedback：
                    {sanitize_string(feedback)}
                    Current Rust Code：
                    {sanitize_string(rust_code)}
                    Ensure that no new bugs are introduced.
                    Please strictly follow the feedback to modify the code to ensure that the Rust output is identical to the C output.
                    Pay special attention to formatting, precision, line breaks and other details.
                    Only the optimized Rust code will be returned, no additional explanation is needed.
                    """

                    optimized = optimize_agent.generate_response(mismatch_optimize_input)
                    new_rust_code = extract_rust_code(optimized)

                    if new_rust_code.strip():
                        rust_code = new_rust_code
                        print(f"代码已针对输出不匹配进行优化 #{extra_attempt}")
                        compile_success, error_or_mismatch_info = compile_and_test_rust(rust_code, c_output_file,
                                                                                        rust_output_file)
                        if compile_success and not error_or_mismatch_info:
                            print("额外优化成功解决了输出不匹配问题")
                            return rust_code
                    else:
                        print("警告：代码优化专家没有返回有效的Rust代码。保持原代码不变。")
                print("额外优化尝试未能解决输出不匹配问题")
            else:
                print("达到最大编译尝试次数")
            break

        print("编译失败或输出不匹配，继续优化")
        feedback_input = f"""
        Analyze the following compilation error or output mismatch issue:
        Issue type: [Compilation Error/Output Mismatch]
        Issue description:
        {sanitize_string(error_or_mismatch_info)}
        Current Rust code:
        {sanitize_string(rust_code)}
        Please provide specific fix suggestions, but do not generate improved code. If it's an output mismatch issue, please analyze the differences between C and Rust outputs, and provide specific modification suggestions to make the outputs consistent.
        """

        feedback = feedback_agent.generate_response(feedback_input)
        optimize_input = f"""
        Optimize the Rust code based on the following specific feedback:
        Feedback:
        {sanitize_string(feedback)}
        Current Rust code:
        {sanitize_string(rust_code)}
        Ensure that no new bugs are introduced.
        Please strictly follow the steps mentioned in the prompt to optimize the code. 
        Ensure all issues mentioned in the feedback are resolved, and add comments for each modification explaining the reason.
        Only return the complete optimized Rust code without additional explanations.
        """

        optimized = optimize_agent.generate_response(optimize_input)
        new_rust_code = extract_rust_code(optimized)

        if new_rust_code.strip():
            rust_code = new_rust_code
            print(f"代码已优化 #{compile_attempt}")
        else:
            print("警告：代码优化专家没有返回有效的Rust代码。保持原代码不变。")

    print("转换未完全成功，但这是最后的结果")
    return rust_code

def extract_rust_code(review_text: str) -> str:
    if review_text is None:
        print("警告：从该专家收到空响应")
        return ""
    code_blocks = re.findall(r'```rust\n(.*?)```', review_text, re.DOTALL)
    if code_blocks:
        return code_blocks[0].strip()
    print("警告：无法从该专家的回复中提取 Rust 代码")
    return ""

def process_files():
    c_code_dir = "test1\c_codes"
    c_out_dir = "test1\c_outputs"
    rust_code_dir = "test1\Translate_Rust_codes"
    rust_out_dir = "test1\Translate_Rust_outputs"

    # 创建新的结果文件夹
    succeed_rust_dir = os.path.join(rust_code_dir, "succeed")
    succeed_out_dir = os.path.join(rust_out_dir, "succeed")
    mismatch_rust_dir = os.path.join(rust_code_dir, "mismatch")
    mismatch_out_dir = os.path.join(rust_out_dir, "mismatch")
    not_compile_dir = os.path.join(rust_code_dir, "not_compile")

    os.makedirs(succeed_rust_dir, exist_ok=True)
    os.makedirs(succeed_out_dir, exist_ok=True)
    os.makedirs(mismatch_rust_dir, exist_ok=True)
    os.makedirs(mismatch_out_dir, exist_ok=True)
    os.makedirs(not_compile_dir, exist_ok=True)

    successful_conversions = 0
    compile_failures = 0
    mismatch_failures = 0
    total_files = 0

    for problem_folder in os.listdir(c_code_dir):
        problem_path = os.path.join(c_code_dir, problem_folder)
        if not os.path.isdir(problem_path):
            continue

        total_files += 1
        c_file_path = os.path.join(problem_path, "main.c")
        c_out_file_path = os.path.join(c_out_dir, f"{problem_folder}.out")

        # 直接在临时位置创建 Rust 文件和输出文件
        temp_rust_file_path = os.path.join(rust_code_dir, f"{problem_folder}.rs")
        temp_rust_out_file_path = os.path.join(rust_out_dir, f"{problem_folder}.out")
        if not os.path.exists(c_file_path):
            print(f"C 文件不存在: {c_file_path}")
            continue
        c_code = read_file_with_auto_encoding(c_file_path)
        print(f"开始转换 {problem_folder}")
        rust_code = convert_c_to_rust(c_code, c_out_file_path, temp_rust_out_file_path)
        # 保存转换后的代码到临时位置
        write_file_with_utf8(temp_rust_file_path, rust_code)
        # 进行编译和测试
        compile_success, mismatch_info = compile_and_test_rust(rust_code, c_out_file_path, temp_rust_out_file_path)

        if compile_success and not mismatch_info:
            successful_conversions += 1
            print(f"成功转换 {problem_folder}")
            # 移动成功的文件到 succeed 文件夹
            shutil.move(temp_rust_file_path, os.path.join(succeed_rust_dir, f"{problem_folder}.rs"))
            shutil.move(temp_rust_out_file_path, os.path.join(succeed_out_dir, f"{problem_folder}.out"))
        elif compile_success:
            mismatch_failures += 1
            print(f"转换 {problem_folder} 失败：输出不匹配")
            # 移动不匹配的文件到 mismatch 文件夹
            shutil.move(temp_rust_file_path, os.path.join(mismatch_rust_dir, f"{problem_folder}.rs"))
            shutil.move(temp_rust_out_file_path, os.path.join(mismatch_out_dir, f"{problem_folder}.out"))
        else:
            compile_failures += 1
            print(f"转换 {problem_folder} 失败：编译错误")
            # 移动编译失败的文件到 not_compile 文件夹
            shutil.move(temp_rust_file_path, os.path.join(not_compile_dir, f"{problem_folder}.rs"))
            # 如果编译失败时创建了输出文件，也需要删除它
            if os.path.exists(temp_rust_out_file_path):
                os.remove(temp_rust_out_file_path)

    print("\n转换统计:")
    print(f"总文件数: {total_files}")
    print(f"成功转换数: {successful_conversions}")
    print(f"编译失败数: {compile_failures}")
    print(f"输出不匹配数: {mismatch_failures}")
    print(f"转换成功率: {successful_conversions / total_files:.2%}")

if __name__ == "__main__":
    process_files()