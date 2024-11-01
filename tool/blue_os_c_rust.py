import os
import re
import shutil
import tempfile
import subprocess
import sys
import chardet
import unicodedata
import argparse

from Agents import *
from tree_sitter_analyzer import analyze_directory, get_translation_order, extract_test_functions, extract_func_dependencies, head_info_extraction
from preprocess.c_code_preprocess import preprocess
from c_code_decomposition import decompose, code_decomposition
# 设置默认编码为UTF-8
sys.stdout.reconfigure(encoding='utf-8')
sys.stderr.reconfigure(encoding='utf-8')

def normalize_string(s):
    '''
    忽略大小写，去除所有空格，格式化浮点数并去除多余0.
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

def read_file_with_auto_encoding(file_path):
    with open(file_path, 'rb') as file:
        raw_data = file.read()
    encoding = chardet.detect(raw_data)['encoding']
    with open(file_path, 'r', encoding=encoding, errors='replace') as f:
        return f.read()

def write_file_with_utf8(file_path, content):
    with open(file_path, 'w', encoding='utf-8', errors='ignore') as file:
        file.write(content)

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
        
        if os.path.exists(rust_pdb):
            os.remove(rust_pdb)
        if os.path.exists(rust_exe):
            os.remove(rust_exe)

        issues = []

        # 解析 clippy 输出，只关注错误
        clippy_output = clippy_result.stdout + clippy_result.stderr
        error_blocks = re.findall(r'error.*?\n\n', clippy_output, re.DOTALL)
        for block in error_blocks:
            issues.append(block.strip())

        # # 提取错误代码并获取详细说明
        # error_codes = set(re.findall(r'error\[E(\d+)]', clippy_output))
        # for code in error_codes:
        #     explain_result = subprocess.run(["rustc", "--explain", f"E{code}"],
        #                                     capture_output=True, text=True, encoding="utf-8")
        #     explanation = explain_result.stdout.strip()
        #     issues.append(f"错误 E{code} 的详细说明:\n{explanation}")

        # 添加总结性错误信息
        summary_match = re.search(r'error: aborting due to (\d+) previous errors', clippy_output)
        if summary_match:
            error_count = summary_match.group(1)
            issues.append(f"总计: {error_count} 个错误")

        return "\n\n".join(issues) if issues else ""

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
            return True, f"输出不匹配。\nC 输出:\n{c_output}\nRust Outputs:\n{rust_output}"

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

    max_static_analysis_and_test_attempts = 7
    static_analysis_and_test_count = 0

    while static_analysis_and_test_count < max_static_analysis_and_test_attempts:
        # 如果静态分析次数未达到阈值，进行静态分析
        static_analysis_and_test_count += 1
        print(f"静态分析尝试 #{static_analysis_and_test_count}")
        analysis_result = static_analysis(rust_code)

        if analysis_result:
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
            Please strictly follow the steps mentioned in the prompt to optimize the code. 
            Ensure all issues mentioned in the feedback are resolved, and add comments for each modification explaining the reason.
            Only return the complete optimized Rust code without additional explanations.
            """
            optimized = optimize_agent.generate_response(optimize_input)
            new_rust_code = extract_rust_code(optimized)
            if new_rust_code.strip():
                rust_code = new_rust_code
                print(f"代码已针对静态分析进行优化 #{static_analysis_and_test_count}\n")
            else:
                print("警告：代码优化专家没有返回有效的Rust代码。保持原代码不变。")

            continue  # 优化后重新进行静态分析
        
        # 进行编译和测试
        print("静态分析未发现错误，进入编译和测试阶段。")
        print(f"编译和测试尝试")
        compile_success, error_or_mismatch_info = compile_and_test_rust(rust_code, c_output_file, rust_code_file, rust_output_file)

        if compile_success and not error_or_mismatch_info:
            print("编译和测试成功")
            print(f"优化迭代次数 #{static_analysis_and_test_count}")
            return rust_code

        # 编译失败或输出不匹配，进行优化
        print("编译失败或者输出不匹配，继续优化")
        feedback_input = f"""
        Analyze the following compilation error:
        Issue description:
        {sanitize_string(error_or_mismatch_info)}
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
        Please strictly follow the steps mentioned in the prompt to optimize the code. 
        Ensure all issues mentioned in the feedback are resolved, and add comments for each modification explaining the reason.
        Only return the complete optimized Rust code without additional explanations.
        """
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
    parser = argparse.ArgumentParser(description='Process C files and convert them to Rust.')
    parser.add_argument('--c_code_dir', default='./Input/01-Primary', type=str, help='Directory path to analyze')
    args = parser.parse_args()

    # 创建新的结果文件夹
    rust_code_dir = "Output/01-Primary/Translate_Rust_codes"
    succeed_rust_dir = os.path.join(rust_code_dir, "succeed")
    not_compile_dir = os.path.join(rust_code_dir, "not_compile")    
    mismatch_rust_dir = os.path.join(rust_code_dir, "mismatch")
    os.makedirs(succeed_rust_dir, exist_ok=True)
    os.makedirs(mismatch_rust_dir, exist_ok=True)
    os.makedirs(not_compile_dir, exist_ok=True)

    successful_conversions = 0
    compile_failures = 0
    mismatch_failures = 0
    total_files = 0

    # Get dependencies and suggested translation order
    dependencies = analyze_directory(args.c_code_dir)
    translation_order = get_translation_order(dependencies)

    for problem_folder in translation_order:
        if not problem_folder.startswith("test"):
            continue
        
        problem_path = os.path.join(args.c_code_dir, problem_folder)
        test_funcs = extract_test_functions(problem_path)
        for test_func in test_funcs:
            # todo: 递归函数，按顺序返回依赖文件中的函数
            # 目前只实现了test_func依赖的查找，并未实现test_func依赖的依赖文件的查找
            depend_files_and_funcs = extract_func_dependencies(args.c_code_dir, problem_folder, test_func)
            for depend_file, depend_funcs in depend_files_and_funcs.items():
                # 对depend_file进行分割，再进行函数级翻译
                funcs_codes = code_decomposition(args.c_code_dir, depend_file, depend_funcs)

                # todo: 提供上下文
                # 目前想法：head_infos作为funcs_codes翻译过程中的上下文
                # 具体每个函数翻译时的内部调用关系，需要额外上下文
                head_infos = head_info_extraction(args.c_code_dir, depend_file)
                # todo:修改convert_c_to_rust函数，
                rust_code = convert_c_to_rust(funcs_codes, head_infos)

                # todo: 先静态分析
                if rust_code:
                    static_errors = static_analysis(rust_code)
                    if static_errors:
                        print(f"{depend_file} 静态分析错误:\n {static_errors}")
                        # todo: 修复规划和修复

                    # todo: 静态分析通过，更新上下文
                    write_file_with_utf8(os.path.join(succeed_rust_dir, f"{depend_file}.rs"), rust_code)

            # todo: 测试函数的所有依赖项翻译完毕，对测试函数进行翻译
            # todo: 提供上下文
            rust_test = convert_c_to_rust(test_func, context)

            # todo: 构建测试环境
            dynamic_errors = compile_and_test_rust(rust_test)
            if dynamic_errors:
                print(f"{test_func} 动态测试错误:\n {dynamic_errors}")
                # todo: 错误定位
                # todo: 修复规划和修复
            
            # todo: 动态测试通过，写入文件，更新上下文
            write_file_with_utf8(os.path.join(succeed_rust_dir, f"{test_func}.rs"), rust_test)

if __name__ == "__main__":
    process_files()