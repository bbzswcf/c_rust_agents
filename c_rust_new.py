import os
import re
import shutil
import tempfile
from Agents import *
from new_prompts import *
import subprocess
import sys
import chardet
import unicodedata
# 设置默认编码为UTF-8
sys.stdout.reconfigure(encoding='utf-8')
sys.stderr.reconfigure(encoding='utf-8')

DEBUG = True
def DEBUG_PRINT(message):
    if DEBUG:
        print(message)

fix_prompt="""
You are given the below errors from clippy and Rust code snippets from one or more '. rs ' files related to the errors.
errors:
{error_block}
---
code snippets:
{code_snippets}
Instructions: 
Fix the error on the above code snippets. 
Not every snippet might require a fix or be relevant to the error, but take into account the code in all above snippets as it could help you derive the best possible fix.
Assume that the snippets might not be complete and could be missing lines above or below. 
Do not add comments or code that is not necessary to fix the error. 
Do not use unsafe or unstable features ( through '#![ feature (...) ]').
For your answer, return one or more ChangeLog groups, each containing one or more fixes to the above code snippets. 
Each group must be formatted with the below instructions.
Format instructions: 
Each ChangeLog group must start with a description of its included fixes. 
The group must then list one or more pairs of ( OriginalCode , FixedCode ) code snippets. 
Each OriginalCode snippet must list all consecutive original lines of code that must be replaced ( including a few lines before and after the fixes ), 
followed by the FixedCode snippet with all consecutive fixed lines of code that must replace the original lines of code ( including the same few lines before and after the changes ).
In each pair, the OriginalCode and FixedCode snippets must start at the same source code line number N. 
Each listed code line, in both the OriginalCode and FixedCode snippets, must be prefixed with [ N ] that matches the line index N in the above snippets, and then be prefixed with exactly the same whitespace indentation as the original snippets above.
Make sure that OriginalCode snippet and FixedCode snippet correspond to the same range, that is, if this changelog is adopted, then simply replacing OriginalCode snippet with FixedCode snippet is sufficient.
If the OriginalCode should be deleted, then keep the corresponding FixedCode snippets empty.
Do not repeat the position of each OriginalCode snippet.
If there is help information in the error, you can directly apply it.
---
ChangeLog :1 @ <file>
FixDescription : <summary>.
OriginalCode@4 -6:
[4] <white space> <original code line>
[5] <white space> <original code line>
[6] <white space> <original code line>
FixedCode@4 -6:
[4] <white space> <fixed code line>
[5] <white space> <fixed code line>
[6] <white space> <fixed code line>
OriginalCode@9 -10:
[9] <white space> <original code line>
[10] <white space> <original code line>
FixedCode@9 -9:
[9] <white space> <fixed code line>
...
ChangeLog : K@ <file>
FixDescription : <summary>.
OriginalCode@15 -16:
[15] <white space> <original code line>
[16] <white space> <original code line>
FixedCode@15 -17:
[15] <white space> <fixed code line>
[16] <white space> <fixed code line>
[17] <white space> <fixed code line>
OriginalCode@23 -23:
[23] <white space> <original code line>
FixedCode@23 -23:
[23] <white space> <fixed code line>
---
Answer:
"""
api_input_prompt="""
        Extract and convert only the C-specific APIs to their Rust equivalents:
        {c_code}
"""
convert_input_prompt = """
    Convert the following C code to Rust using the provided API mappings:
    C code:
    {c_code}
    API mappings:
    {api_conversion}
    Remember to output only the converted Rust code without any explanations.
"""
# feedback_input_prompt="""
# Original C Code:
# {c_code}

# Translated Rust Code:
# {rust_code}

# Error message:
# {error_msg}

# Issues:
# """

optimize_input_prompt="""
Current Rust Code:
{rust_code}

Fix issues:
{issues}

Fixed Rust Code:
"""

def normalize_string(s):
    '''忽略大小写，去除所有空格，格式化浮点数并去除多余0.
    '''    
    if not re.search(r'\d', s):
        return s
    try:
        float_pattern = re.compile(r'[-+]?\d*\.\d+([eE][-+]?\d+)?')
        matches = list(float_pattern.finditer(s))
        for match in reversed(matches):
            original_float = float(match.group())
            if original_float.is_integer():
                original_float = int(original_float)
            # 将匹配到的浮点数转为浮点型再转回字符串，以去除多余零
            formatted_number = f"{original_float}"
            # 替换字符串中对应的浮点数
            s = s[:match.start()] + formatted_number + s[match.end():]
    except Exception as e:
        print("浮点数格式化发生错误，已返回原字符串")

    s = s.lower()
    s = re.sub(r'\s+', '', s)
    return s

def extract_rust_code(review_text: str) -> str:
    if review_text is None:
        print("警告：从该专家收到空响应")
        return ""
    code_blocks = re.findall(r'```rust\n(.*?)```', review_text, re.DOTALL)
    if code_blocks:
        return code_blocks[0].strip()
    print("警告：无法从该专家的回复中提取 Rust 代码")
    return ""

def sanitize_string(s):
    """
    将字符串中的非ASCII字符替换为它们的ASCII表示或删除。
    """
    return ''.join(c for c in unicodedata.normalize('NFKD', s) if ord(c) < 128)

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

def check_changelog(rust_code_lines, changelog):
    # 正则表达式模式来匹配OriginalCode和FixedCode块
    pattern = re.compile(r'(OriginalCode@.*?)(?=OriginalCode@|$)', re.DOTALL)
    # 查找所有匹配的块
    matches = pattern.findall(changelog)
    # 提取为行号到代码的映射
    rust_lines_map = {int(line.split(']', 1)[0][1:]): line.split(']', 1)[1].strip() for line in rust_code_lines}
    applied_logpairs=[]
    # 遍历每个匹配的块
    for match in matches:
        # 提取OriginalCode和FixedCode
        original_code_match = re.search(r'OriginalCode@.*?:\n(.*?)(?=FixedCode@|$)', match, re.DOTALL)
        fixed_code_match = re.search(r'FixedCode@.*?:\n(.*?)(?=OriginalCode@|$)', match, re.DOTALL)
        
        if original_code_match and fixed_code_match:
            fixed_code = fixed_code_match.group(1).strip()
            original_code = original_code_match.group(1).strip()
            # 提取为行号到代码的映射
            original_lines = re.split(r'(?=\[\d+\])', original_code)
            original_lines = [line.strip() for line in original_lines if line]
            original_lines_map = {int(line.split(']', 1)[0][1:]): line.split(']', 1)[1].strip() for line in original_lines}

            # 检查是否接受这条changelog（original code必须与原代码相同）
            accept = True
            for line_number, original_line in original_lines_map.items():
                if line_number not in rust_lines_map or original_line != rust_lines_map[line_number]:
                    accept = False
                    break
            if accept:
                applied_logpairs.append((original_code, fixed_code))
    return applied_logpairs

def static_analysis(rust_code: str) -> str:
    with tempfile.TemporaryDirectory() as tmpdir:
        cur_dir = os.getcwd()
        rust_pdb = os.path.join(cur_dir, "temp.pdb")
        rust_exe = os.path.join(cur_dir, "temp.exe")
        rust_file = os.path.join(tmpdir, "temp.rs")
        write_file_with_utf8(rust_file, rust_code)

        # 使用 clippy 进行静态分析
        clippy_result = subprocess.run(["clippy-driver", rust_file],
                                       capture_output=True, text=True, encoding="utf-8")

        # if os.path.exists("tmp_proj"):
        #     shutil.rmtree("tmp_proj")
        # subprocess.run(["cargo", "new", "tmp_proj"], check=True)
        # os.chdir(os.path.join(cur_dir, "tmp_proj"))
        # rust_file = os.path.join(cur_dir, "tmp_proj/temp.rs")
        # write_file_with_utf8(rust_file, rust_code)
        # clippy_result = subprocess.run(["cargo", "clippy"],
        #                                capture_output=True, text=True, encoding="utf-8")
        # os.chdir(cur_dir)

        
        
        if os.path.exists(rust_pdb):
            os.remove(rust_pdb)
        if os.path.exists(rust_exe):
            os.remove(rust_exe)

        issues = []

        # 解析 clippy 输出，只关注错误
        clippy_output = clippy_result.stdout + clippy_result.stderr
        print("clippy_output:")
        print(clippy_output)

        error_count = 0
        error_blocks = re.findall(r'error.*?\n\n', clippy_output, re.DOTALL)
        for block in error_blocks:
            if block.startswith("error["):
                # 过滤缺少主函数错误
                if "E0601" in block:
                    continue
                issues.append(block.strip())
                error_count = error_count + 1

        # 提取错误代码并获取详细说明
        # error_codes = set(re.findall(r'error\[E(\d+)]', clippy_output))
        # for code in error_codes:
        #     explain_result = subprocess.run(["rustc", "--explain", f"E{code}"],
        #                                     capture_output=True, text=True, encoding="utf-8")
        #     explanation = explain_result.stdout.strip()
        #     issues.append(f"错误 E{code} 的详细说明:\n{explanation}")

        # 添加总结性错误信息
        summary_match = re.search(r'error: aborting due to (\d+) previous errors', clippy_output)
        # if summary_match:
        #     # error_count = summary_match.group(1)
        #     issues.append(f"总计: {error_count} 个错误")
        return "\n\n".join(issues) if issues else "", issues

def compile(rust_code: str) -> tuple[bool, str]:
    current_dir = os.getcwd()
    rust_file = os.path.join(current_dir, "temp_main.rs")
    rust_exe = os.path.join(current_dir, "temp_main.exe" if sys.platform == "win32" else "temp_main")
    rust_pdb = os.path.join(current_dir, "temp_main.pdb")

    try:
        write_file_with_utf8(rust_file, rust_code)
        print(f"Rust 代码已写入: {rust_file}")
        # 添加 -A dead_code 和 -A unused_variables 标志来忽略未使用变量的警告
        compile_command = ["rustc", "-A", "dead_code", "-A", "unused_variables", rust_file, "-o", rust_exe]
        compile_result = subprocess.run(compile_command, capture_output=True, text=True, encoding="utf-8")

        if compile_result.returncode != 0:
            return False, f"Rust 编译失败:\n{compile_result.stderr}"
        else:
            return True, ""

    finally:
        if os.path.exists(rust_file):
            os.remove(rust_file)
        if os.path.exists(rust_exe):
            os.remove(rust_exe)
        if os.path.exists(rust_pdb):
            os.remove(rust_pdb)

def compile_and_test_rust(rust_code: str, c_output_file: str, rust_code_file: str, rust_output_file: str) -> tuple[bool, str]:
    current_dir = os.getcwd()
    rust_file = os.path.join(current_dir, "temp_main.rs")
    rust_exe = os.path.join(current_dir, "temp_main.exe" if sys.platform == "win32" else "temp_main")
    rust_pdb = os.path.join(current_dir, "temp_main.pdb")

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

        rust_output = run_result.stdout.strip()
        write_file_with_utf8(rust_output_file, rust_output)
        write_file_with_utf8(rust_code_file, rust_code)

        c_output = read_file_with_auto_encoding(c_output_file).strip()

        # if rust_output.strip() == c_output.strip():
        if normalize_string(rust_output) == normalize_string(c_output):
            return True, ""
        else:
            return True, f"Output mismatch:\nC output:\n{c_output}\nRust output:\n{rust_output}"

    finally:
        if os.path.exists(rust_file):
            os.remove(rust_file)
        if os.path.exists(rust_exe):
            os.remove(rust_exe)
        if os.path.exists(rust_pdb):
            os.remove(rust_pdb)

def convert_c_to_rust(c_code: str, c_output_file: str, rust_code_file:str, rust_output_file: str) -> str:
    if c_code is None or c_code.strip() == "":
        print("错误：没有提供有效的 C 代码进行转换")
        return ""

    print("开始API转换")
    api_conversion = api_agent.generate_response(
        api_input_prompt.format(c_code=c_code)
    )

    print("开始语法转换")
    combined_syntax_input = convert_input_prompt.format(c_code=c_code, api_conversion=api_conversion)
    rust_code = syntax_agent.generate_response(combined_syntax_input)
    rust_code = extract_rust_code(rust_code)

    max_static_analysis_and_test_attempts = 10
    static_analysis_and_test_count = 0

    while static_analysis_and_test_count < max_static_analysis_and_test_attempts:
        # 如果静态分析次数未达到阈值，进行静态分析
        static_analysis_and_test_count += 1
        print(f"静态分析尝试 #{static_analysis_and_test_count}")
        analysis_result, issues = static_analysis(rust_code)

        if analysis_result:
            print("发现静态分析问题，生成fix log...")
            # block_1 = issues[0]
            # print(f"选择：{block_1}")

            # 正则表达式，匹配双引号内的内容，并替换换行符为特殊标记
            modified_str = re.sub(r'(".*?)(\n)(.*?")', lambda m: m.group(0).replace('\n', '___NEWLINE___'), rust_code)
            # 按行分割字符串
            lines = [line for line in modified_str.splitlines() if line]
            # 恢复双引号内的特殊标记
            lines = [line.replace('___NEWLINE___', '\n') for line in lines]
            # 给每一行前加上行号
            numbered_lines = [f"[{index + 1}] {line}" for index, line in enumerate(lines)]
            # 将带有行号的行合并回一个字符串
            numbered_rust_code = '\n'.join(numbered_lines)

            fix_input = fix_prompt.format(error_block=analysis_result, code_snippets=numbered_rust_code)
            DEBUG_PRINT("------------fix_input------------")

            fix_log = fix_agent.generate_response(fix_input)
            print("修复日志：")
            print(fix_log)

            print("------------修复前------------")
            print(numbered_rust_code)
            # print(modified_str)

            applied_logpairs = check_changelog(numbered_lines, fix_log)
            print(f"经检查{len(applied_logpairs)}条changelog被采用，开始修复")
            for (original_code, fixed_code) in applied_logpairs:
                DEBUG_PRINT("-" * 40)
                print("Original Code:")
                print(original_code)
                print("Fixed Code:")
                print(fixed_code)
                numbered_rust_code = numbered_rust_code.replace(original_code, fixed_code)
                DEBUG_PRINT("------应用修复------")
                DEBUG_PRINT(numbered_rust_code)
            print("------------修复后------------")
            print(numbered_rust_code)

            # 正则表达式，匹配双引号内的内容，并替换换行符为特殊标记
            modified_str = re.sub(r'(".*?)(\n)(.*?")', lambda m: m.group(0).replace('\n', '___NEWLINE___'), numbered_rust_code)
            # 按行分割字符串
            lines = [line for line in modified_str.splitlines() if line]
            # 恢复双引号内的特殊标记
            lines = [line.replace('___NEWLINE___', '\n') for line in lines]
            # 去除行号
            lines = [re.sub(r'^\[\w+\] ', '', line) for line in lines]
            # 合并回代码
            new_rust_code = '\n'.join(lines)

            if new_rust_code.strip():
                rust_code = new_rust_code
                print(f"代码已针对静态分析进行优化 #{static_analysis_and_test_count}\n")
            else:
                print("警告：代码优化专家没有返回有效的Rust代码。保持原代码不变。")

            continue  # 优化后重新进行静态分析

        print("静态分析未发现错误，进入编译和测试阶段。")
        # 进行编译和测试
        # print("编译尝试")
        # compile_success, error_or_mismatch_info = compile(rust_code)

        # if not compile_success:
        #     print("编译失败，开始优化")
        #     fix_input = compiler_error_prompt_0+compiler_error_prompt_1.format(c_code=c_code, rust_code=rust_code, error_message=error_or_mismatch_info)
        #     print("修复prompt：")
        #     print(fix_input)
        #     fix_plan = feedback_agent.generate_response(fix_input)
        #     print("修复计划：")
        #     print(fix_plan)
        #     optimize_input=optimize_input_prompt.format(rust_code=rust_code, issues=fix_plan)
        #     print("优化prompt：")
        #     print(optimize_input)
        #     optimized = optimize_agent.generate_response(optimize_input)
        #     new_rust_code = extract_rust_code(optimized)

        #     if new_rust_code.strip():
        #         rust_code = new_rust_code
        #         print(f"代码已针对静态分析进行优化 #{static_analysis_and_test_count}\n")
        #     else:
        #         print("警告：代码优化专家没有返回有效的Rust代码。保持原代码不变。")

        #     continue  # 优化后重新进行静态分析

        # print("编译成功")
        print(f"运行测试尝试")
        compile_success, error_or_mismatch_info = compile_and_test_rust(rust_code, c_output_file, rust_code_file, rust_output_file)

        if compile_success and not error_or_mismatch_info:
            print("编译和测试成功")
            print(f"优化迭代次数 #{static_analysis_and_test_count}")
            return rust_code

        # 编译失败或输出不匹配，进行优化
        print("输出不匹配，继续优化")
        DEBUG_PRINT("编译测试错误信息：")
        DEBUG_PRINT(error_or_mismatch_info)
        if "运行失败" in error_or_mismatch_info:
            print("运行失败，开始优化")
            fix_input = compiler_error_prompt_0+compiler_error_prompt_1.format(c_code=c_code, rust_code=rust_code, error_message=error_or_mismatch_info)
            # print("修复prompt：")
            # print(fix_input)
            fix_plan = feedback_agent.generate_response(fix_input)
            print("修复计划：")
            print(fix_plan)
            optimize_input=optimize_input_prompt.format(rust_code=rust_code, issues=fix_plan)
            # print("优化prompt：")
            # print(optimize_input)
            optimized = optimize_agent.generate_response(optimize_input)
            new_rust_code = extract_rust_code(optimized)

            if new_rust_code.strip():
                rust_code = new_rust_code
                print(f"代码已针对静态分析进行优化 #{static_analysis_and_test_count}\n")
            else:
                print("警告：代码优化专家没有返回有效的Rust代码。保持原代码不变。")

            continue  # 优化后重新进行静态分析

        elif "Output mismatch" in error_or_mismatch_info:
            align_input=align_prompt_0+align_prompt_1.format(c_code=c_code, rust_code=rust_code)
            # print("对齐prompt：")
            # print(align_input)
            align_result=feedback_agent.generate_response(align_input)
            # print("对齐输出：")
            # print(align_result)
            # 提取C代码
            c_start = align_result.find("### C")
            c_end = align_result.find("### Rust")
            aligned_c_code = align_result[c_start+1:c_end].strip()
            # 提取Rust代码
            rust_start = align_result.find("### Rust")
            aligned_rust_code = align_result[rust_start+1:].strip()

            fix_input=consistency_analyze_and_fix_prompt_0+consistency_analyze_and_fix_prompt_1.format(aligend_c_code=aligned_c_code,aligend_rust_code=aligned_rust_code, error_message=error_or_mismatch_info)
            print("修复prompt：")
            print(fix_input)
            fix_plan= feedback = feedback_agent.generate_response(fix_input)
            print("修复计划：")
            print(fix_plan)

            optimize_input=optimize_input_prompt.format(rust_code=rust_code, issues=fix_plan)
            optimized = optimize_agent.generate_response(optimize_input)
            new_rust_code = extract_rust_code(optimized)

            if new_rust_code.strip():
                rust_code = new_rust_code
                print(f"代码已针对编译或输出不匹配进行优化 #{static_analysis_and_test_count}")
            else:
                print("警告：代码优化专家没有返回有效的Rust代码。保持原代码不变。")
        # 优化后重新进行循环，如果未达到阈值，将重新进行静态分析

    print("达到最大静态分析和测试次数，转换未完全成功，但这是最后的结果")
    return rust_code

def process_files():
    c_code_dir = "test1\c_codes"
    c_out_dir = "test1\c_outputs"
    rust_code_dir = "test1\Translate_Rust_codes"
    rust_out_dir = "test1\Translate_Rust_outputs"

    # c_code_dir = "test2\c_codes"
    # c_out_dir = "test2\c_outputs"
    # rust_code_dir = "test2\Translate_Rust_codes"
    # rust_out_dir = "test2\Translate_Rust_outputs"

    # c_code_dir = "testfix\c_codes"
    # c_out_dir = "testfix\c_outputs"
    # rust_code_dir = "testfix\Translate_Rust_codes"
    # rust_out_dir = "testfix\Translate_Rust_outputs"

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
        rust_code = convert_c_to_rust(c_code, c_out_file_path, temp_rust_file_path, temp_rust_out_file_path)
        if not os.path.exists(temp_rust_file_path):
            write_file_with_utf8(temp_rust_file_path, rust_code)
        
        # 进行编译和测试
        compile_success, mismatch_info = compile_and_test_rust(rust_code, c_out_file_path, temp_rust_file_path, temp_rust_out_file_path)

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
            print(f"转换 {problem_folder} 失败：编译错误或运行失败")
            # 保留最后一次可以成功运行的rust代码和运行结果
            shutil.move(temp_rust_file_path, os.path.join(not_compile_dir, f"{problem_folder}.rs"))
            if os.path.exists(temp_rust_out_file_path):
                shutil.move(temp_rust_out_file_path, os.path.join(not_compile_dir, f"{problem_folder}.out"))
            write_file_with_utf8(os.path.join(not_compile_dir, f"{problem_folder}_compile_error.out"), mismatch_info)

    print("\n转换统计:")
    print(f"总文件数: {total_files}")
    print(f"成功转换数: {successful_conversions}")
    print(f"编译失败数: {compile_failures}")
    print(f"输出不匹配数: {mismatch_failures}")
    print(f"转换成功率: {successful_conversions / total_files:.2%}")

if __name__ == "__main__":
    process_files()