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
from tree_sitter_analyzer import analyze_directory, get_translation_order, extract_test_functions, extract_func_dependencies, head_info_extraction, extract_func_calls, extract_all_funcs
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

def insert_file_with_utf8(file_path, content):
    with open(file_path, 'a', encoding='utf-8', errors='ignore') as file:
        file.write(content)

def static_analysis(rust_code: str) -> str:
    # todo: 重新进行修改, 传入文件路径, 直接在文件上进行静态分析
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

def compile_and_test_rust(rust_code: str, test_func: str, test_problem_name: str, rust_result_dir: str) -> tuple[bool, str]:
    current_dir = os.getcwd()
    cargo_toml = os.path.join(current_dir, os.path.join(rust_result_dir, "Cargo.toml"))

    try:
        compile_command = ["cargo", "test", test_func, "--manifest-path", cargo_toml, "--test", test_problem_name]
        compile_result = subprocess.run(compile_command, capture_output=True, text=True, encoding="utf-8")

        if compile_result.returncode != 0:
            return False, f"Rust 编译失败:\n{compile_result.stderr}"
        
        if "test result: ok" not in compile_result.stdout:
            return False, f"Rust 测试失败:\n{compile_result.stdout}\n{compile_result.stderr}"
        
        return True, ""
    except Exception as e:
        return False, f"Rust 测试失败:\n{str(e)}"

def convert_c_to_rust(c_codes: list[str], head_infos: dict, rust_code_file:str, depend_files: list[str], c_to_rust_mappings: dict) -> str:
    if not c_codes:
        print("错误：没有提供有效的 C 代码进行转换")
        return ""

    rust_codes = []
    for c_code in c_codes:
        
        print("开始API转换")
        api_conversion = api_agent.generate_response(
        f"""
        Extract and convert only the C-specific APIs to their Rust equivalents:
        {c_code}
        """
        )

        print("开始语法转换")
        combined_syntax_input = f"""
        Convert the following C code to Rust using the provided API mappings and function calls:
        C code:
        {c_code}"""
       
        if head_infos:
            for head_file, head_codes in head_infos.items():
                combined_syntax_input += f"""
                Included headers:
                <{head_file}>:
                {head_codes}
                """
            # 只在第一次翻译（既翻译文件中函数之前的代码内容）时，将头文件中的信息一并翻译
            head_infos = {}
        else:
            # 提取出函数调用，检索c_to_rust_mappings对应的rust函数, 放入语法专家prompt中
            func_calls = extract_func_calls(c_code)
            C_func_calls = []
            Rust_func_calls = []
            for depend_file in depend_files:
                depend_file_name = os.path.splitext(os.path.basename(depend_file))[0]
                
                Rust_func_calls = [c_to_rust_mappings[depend_file_name].get(func) 
                                    for func in func_calls 
                                    if func in c_to_rust_mappings[depend_file_name]]
                C_func_calls = [func for func in func_calls 
                                if func in c_to_rust_mappings[depend_file_name]]

            combined_syntax_input += f"""
            function calls:"""
            for c_func, rust_func in zip(C_func_calls, Rust_func_calls):
                if rust_func:
                    combined_syntax_input += f"""
                    {c_func} -> {rust_func}
                    """

        combined_syntax_input += f"""
        API mappings:
        {api_conversion}
        Remember to output only the converted Rust code without any explanations.
        Declare all items(strctures, enums, functions, constants, etc.) using pub(public) to allow importing.
        """

        rust_code = syntax_agent.generate_response(combined_syntax_input)
        rust_code = extract_rust_code(rust_code)

        max_static_analysis_and_test_attempts = 7
        static_analysis_and_test_count = 0

        while static_analysis_and_test_count < max_static_analysis_and_test_attempts:
            # 如果静态分析次数未达到阈值，进行静态分析
            static_analysis_and_test_count += 1
            print(f"静态分析尝试 #{static_analysis_and_test_count}")
            analysis_result = static_analysis("\n".join(rust_codes) + "\n" + rust_code)

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
            else:
                insert_file_with_utf8(rust_code_file, rust_code)
                rust_codes.append(rust_code)
                # Extract function signatures from the rust code including parameters
                func_signatures = re.findall(r'pub\s+fn\s+([a-zA-Z0-9_]+\s*\([^)]*\))', rust_code)
                # todo: 更新c_to_rust_mappings
                break

    return "\n".join(rust_codes)

def process_files():
    parser = argparse.ArgumentParser(description='Process C files and convert them to Rust.')
    parser.add_argument('--c_code_dir', default='./Input/01-Primary', type=str, help='Directory path to analyze')
    args = parser.parse_args()

    # 创建新的结果文件夹
    rust_result_dir = "Output/primary"
    rust_code_dir = os.path.join(rust_result_dir, "src")
    rust_test_dir = os.path.join(rust_result_dir, "test")

    successful_conversions = 0
    compile_failures = 0
    mismatch_failures = 0
    total_files = 0
    # translated_bytes keeps track of which portions of each file have already been translated
    translated_bytes = {}

    # Get dependencies and suggested translation order
    dependencies = analyze_directory(args.c_code_dir)
    translation_order = get_translation_order(dependencies)
    # Initialize dictionary to track C to Rust function and virable name mappings for each file
    c_to_rust_mappings = {os.path.splitext(os.path.basename(file))[0]: {} for file in translation_order}
    # Initialize function mappings for each file
    for file in translation_order:
        c_funcs = extract_all_funcs(os.path.join(args.c_code_dir, file))
        for func in c_funcs:
            c_to_rust_mappings[os.path.splitext(os.path.basename(file))[0]][func] = ''

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
                temp_rust_file_path = os.path.join(rust_code_dir, f"{os.path.splitext(depend_file)[0]}.rs")

                # 对depend_file进行分割，再进行函数级翻译
                start_byte = 0
                if os.path.splitext(depend_file)[0] in translated_bytes:
                    start_byte = translated_bytes[os.path.splitext(depend_file)[0]]
                funcs_codes, max_end_byte = code_decomposition(args.c_code_dir, depend_file, depend_funcs, start_byte)
                translated_bytes[os.path.splitext(depend_file)[0]] = max_end_byte

                # 对头文件中的信息也进行翻译
                head_infos = {}
                if start_byte == 0:
                    head_infos = head_info_extraction(args.c_code_dir, depend_file)
                depend_files = dependencies['src/'+os.path.splitext(depend_file)[0]+'.c']
                depend_files.append(os.path.splitext(depend_file)[0])
                rust_code = convert_c_to_rust(funcs_codes, head_infos, temp_rust_file_path, depend_files, c_to_rust_mappings)

            print(f"测试文件{problem_folder}的测试函数 {test_func} 的依赖项翻译完毕，开始翻译测试函数")
            test_problem_name = os.path.splitext(os.path.basename(problem_folder.replace('-', '_')))[0]
            temp_rust_test_file_path = os.path.join(rust_test_dir, f"{test_problem_name}.rs")

            start_byte = 0
            if test_problem_name in translated_bytes:
                start_byte = translated_bytes[test_problem_name]
            funcs_codes, max_end_byte = code_decomposition(args.c_code_dir, test_problem_name, test_func, start_byte)
            translated_bytes[test_problem_name] = max_end_byte
            rust_test = convert_c_to_rust(funcs_codes, {}, temp_rust_test_file_path, c_to_rust_mappings)

            print(f"测试文件{problem_folder}的测试函数 {test_func} 的翻译完毕，开始动态测试")
            dynamic_errors = compile_and_test_rust(rust_test, test_func, test_problem_name, rust_result_dir)
            if dynamic_errors:
                print(f"{test_func} 动态测试错误:\n {dynamic_errors}")
                # todo: 错误定位
                # todo: 修复规划和修复
            

if __name__ == "__main__":
    process_files()