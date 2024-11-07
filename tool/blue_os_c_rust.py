import os
import re
import time
import subprocess
import sys
import chardet
import unicodedata
import argparse
import logging

from Agents import *
from tree_sitter_analyzer import (
    analyze_directory,
    get_translation_order,
    extract_test_functions,
    extract_func_dependencies,
    head_info_extraction,
    extract_func_calls,
    extract_all_funcs
)
from c_code_decomposition import code_decomposition

# 设置默认编码为UTF-8
sys.stdout.reconfigure(encoding='utf-8')
sys.stderr.reconfigure(encoding='utf-8')

# 配置日志
def setup_logging():
    os.makedirs('logs', exist_ok=True)
    
    # 配置日志格式
    log_format = '%(asctime)s - %(levelname)s - %(message)s'
    date_format = '%Y-%m-%d %H:%M:%S'
    
    # 创建日志文件名（使用时间戳）
    log_file = f'logs/conversion_{time.strftime("%Y%m%d_%H%M%S")}.log'
    
    logging.basicConfig(
        level=logging.INFO,
        format=log_format,
        datefmt=date_format,
        handlers=[
            logging.FileHandler(log_file, encoding='utf-8'),  # 文件处理器
            logging.StreamHandler()  # 控制台处理器
        ]
    )


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
        logging.info("浮点数格式化发生错误，已返回原字符串")

    s = s.lower()
    s = re.sub(r'\s+', '', s)
    return s


def extract_rust_code(review_text: str) -> str:
    if review_text is None:
        logging.info("警告：从该专家收到空响应")
        return ""
    code_blocks = re.findall(r'```rust\n(.*?)```', review_text, re.DOTALL)
    if code_blocks:
        return code_blocks[0].strip()
    logging.info("警告：无法从该专家的回复中提取 Rust 代码")
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
        file.write('\n'+content)


def static_analysis(rust_result_dir: str, rust_file: str) -> str:
    cur_dir = os.getcwd()
    cargo_toml_dir = os.path.join(os.path.join(cur_dir, rust_result_dir), "Cargo.toml")
    rust_file = os.path.join(cur_dir, rust_file)

    if "test" not in rust_file:
        # 使用 clippy 进行静态分析
        with open(rust_file, 'a', encoding='utf-8') as f:
            f.write("\nfn main() {}")
        clippy_result = subprocess.run(
            ["clippy-driver", rust_file],
            capture_output=True,
            text=True,
            encoding="utf-8"
        )
        # Remove the temporary main function
        content = read_file_with_auto_encoding(rust_file)
        content = content.replace("\nfn main() {}", "")
        write_file_with_utf8(rust_file, content)
    else:
        clippy_result = subprocess.run(
            ["cargo", "clippy", "--manifest-path", cargo_toml_dir],
            capture_output=True,
            text=True,
            encoding="utf-8"
        )
    logging.info("静态分析结果：")
    logging.info(clippy_result.stdout+clippy_result.stderr)
    issues = []

    # 解析 clippy 输出，只关注错误
    clippy_output = clippy_result.stdout + clippy_result.stderr
    error_blocks = re.findall(r'error.*?\n\n', clippy_output, re.DOTALL)
    for block in error_blocks:
        issues.append(block.strip())

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
        logging.info("动态测试结果：")
        logging.info(compile_result.stdout+compile_result.stderr)
        if compile_result.returncode != 0:
            return False, f"Rust 编译失败:\n{compile_result.stderr}"

        if "test result: ok" not in compile_result.stdout:
            return False, f"Rust 测试失败:\n{compile_result.stdout}\n{compile_result.stderr}"

        return True, ""
    except Exception as e:
        return False, f"Rust 测试失败:\n{str(e)}"


def convert_c_funcs_to_rust(c_file: str, c_codes: list[str], rust_code_file: str, depend_files: list[str], 
                            c_to_rust_mappings: dict, func_signatures: list[str], total_funcs: list[str], rust_result_dir: str) -> str:
    if not c_codes:
        return ""

    rust_codes = []
    for c_code in c_codes:
        logging.info("开始API转换")
        api_conversion = api_agent.generate_response(
            f"""\nExtract and convert only the C-specific APIs to their Rust equivalents:\n{c_code}\n"""
        )
        logging.info(api_conversion)
        logging.info("开始语法转换")
        combined_syntax_input = f"""\nConvert the following C code to Rust using the function calls, provided API mappings:\nC code:\n{c_code}\n"""

        # 从c_to_rust_mappings找到对应depend_file中的Rust_signatures
        depend_files_names = [os.path.splitext(os.path.basename(depend_file))[0] for depend_file in depend_files]
        func_calls = extract_func_calls(c_code)
        total_func_names = [sig.split('(')[0].split(' ')[-1].replace('*', '').strip() for sig in total_funcs]
        func_calls = [call for call in func_calls if call in total_func_names]
        c_signatures = []
        rust_signatures = []
        for func_call in func_calls:
            for depend_file_name in depend_files_names:
                for c_func_signature, rust_func_signature in c_to_rust_mappings[depend_file_name].items():
                    if func_call == c_func_signature.split('(')[0].split(' ')[-1].replace('*', '').strip():
                        c_signatures.append(c_func_signature)
                        rust_signatures.append(rust_func_signature)

        # todo: 增加已翻译过的变量、结构体、宏定义作为上下文（防止重复翻译）
        combined_syntax_input += f"""\nfunction calls:\n"""

        for c_func, rust_func in zip(c_signatures, rust_signatures):
            if rust_func:
                combined_syntax_input += f"""\n{c_func} -> {rust_func}\n"""

        combined_syntax_input += f"""\nAPI mappings:\n{api_conversion}\nRemember to output only the converted Rust code without any explanations.\nDeclare all items(strctures, enums, functions, constants, etc.) using pub(public) to allow importing.\nOnly return the function implementation without redefining any structs, variables, or types that are already defined in the codebase.\n"""
        logging.info(f"语法专家prompt: {combined_syntax_input}")

        rust_code = syntax_agent.generate_response(combined_syntax_input)
        logging.info(rust_code)
        rust_code = extract_rust_code(rust_code)
        # Add #[test] attribute if this is a test file
        if "test" in rust_code_file:
            rust_code = "#[test]\n" + rust_code
        insert_file_with_utf8(rust_code_file, rust_code)

        max_static_analysis_and_test_attempts = 7
        static_analysis_count = 0

        while static_analysis_count < max_static_analysis_and_test_attempts:
            # 如果静态分析次数未达到阈值，进行静态分析
            static_analysis_count += 1
            logging.info(f"静态分析尝试 #{static_analysis_count}")
            analysis_result = static_analysis(rust_result_dir, rust_code_file)

            if analysis_result:
                logging.info("发现静态分析问题，正在优化...")
                feedback_input = f"""\nAnalyze the following static analysis results:\nIssue description:\n{sanitize_string(analysis_result)}\nCurrent Rust code:\n{sanitize_string(rust_code)}\nPlease provide specific fix suggestions, but do not generate improved code.\n"""

                logging.info(f"修复规划专家prompt: {feedback_input}")
                feedback = feedback_agent.generate_response(feedback_input)
                logging.info(feedback)

                optimize_input = f"""\nOptimize the Rust code based on the following specific feedback:\nFeedback:\n{sanitize_string(feedback)}\nCurrent Rust code:\n{sanitize_string(rust_code)}\nPlease strictly follow the steps mentioned in the prompt to optimize the code.\nEnsure all issues mentioned in the feedback are resolved, and add comments for each modification explaining the reason.\nOnly return the complete optimized Rust code without additional explanations.\n"""

                logging.info(f"修复专家prompt: {optimize_input}")
                optimized = optimize_agent.generate_response(optimize_input)
                logging.info(optimized)

                new_rust_code = extract_rust_code(optimized)
                if new_rust_code.strip():
                    current_content = read_file_with_auto_encoding(rust_code_file)
                    write_file_with_utf8(rust_code_file, current_content.replace(rust_code, new_rust_code))
                    rust_code = new_rust_code
                    logging.info(f"代码已针对静态分析进行优化 #{static_analysis_count}\n")
                else:
                    logging.info("警告：代码优化专家没有返回有效的Rust代码。保持原代码不变。")
                if static_analysis_count == max_static_analysis_and_test_attempts:
                    logging.info(f"翻译失败")
                    rust_codes.append(rust_code)
                continue  # 优化后重新进行静态分析
            else:
                logging.info("静态分析通过")
                rust_codes.append(rust_code)
                rust_signatures = re.findall(r'pub\s+fn\s+([a-zA-Z0-9_]+\s*\([^)]*\)(?:\s*->\s*[^{]+)?)', rust_code)
                # 更新c_to_rust_mappings
                if rust_signatures:
                    c_func_name = ''
                    for c_func in func_signatures:
                        if c_func in c_code:
                            c_func_name = c_func
                            break
                    if c_func_name:
                        c_to_rust_mappings[c_file][c_func_name] = rust_signatures[0]
                break
        
    return "\n".join(rust_codes)


def convert_c_initialization_to_rust(c_file: str, c_code: str, head_infos: dict, depend_files: list[str], 
                                     c_to_rust_mappings: dict, rust_code_file: str, rust_result_dir: str) -> str:  
    # Remove all #include statements from c_code and add use statements for depend_files
    c_code = re.sub(r'#include\s*[<"].*?[>"]', '', c_code).strip()

    # 清空rust_code_file
    with open(rust_code_file, 'w', encoding='utf-8', errors='ignore') as file:
        file.write("")

    if depend_files:
        for depend_file in depend_files:
            with open(rust_code_file, 'a', encoding='utf-8', errors='ignore') as file:
                file.write(f"use primary::{os.path.basename(depend_file.replace('-', '_'))}::*;\n")

    pre_code = ''
    for head_file, head_codes in head_infos.items():
        pre_code += head_codes.strip()+'\n'
    c_code = pre_code + c_code

    logging.info("开始语法转换")
    combined_syntax_input = f"""\nConvert the following C code to Rust using the provided API mappings:\nC code:\n{c_code}\nRemember to output only the converted Rust code without any explanations.\nDeclare all items(strctures, enums, functions, constants, etc.) using pub(public) to allow importing.\n"""

    rust_code = syntax_agent_2.generate_response(combined_syntax_input)
    logging.info(rust_code)
    rust_code = extract_rust_code(rust_code)
    insert_file_with_utf8(rust_code_file, rust_code)

    max_static_analysis_and_test_attempts = 7
    static_analysis_count = 0

    while static_analysis_count < max_static_analysis_and_test_attempts:
        # 如果静态分析次数未达到阈值，进行静态分析
        static_analysis_count += 1
        logging.info(f"静态分析尝试 #{static_analysis_count}")
        analysis_result = static_analysis(rust_result_dir, rust_code_file)

        if analysis_result:
            logging.info("发现静态分析问题，正在优化...")
            feedback_input = f"""\nAnalyze the following static analysis results:\nIssue description:\n{sanitize_string(analysis_result)}\nCurrent Rust code:\n{sanitize_string(rust_code)}\nPlease provide specific fix suggestions, but do not generate improved code.\n"""

            logging.info(f"修复规划专家prompt: {feedback_input}")
            feedback = feedback_agent.generate_response(feedback_input)
            logging.info(feedback)

            optimize_input = f"""\nOptimize the Rust code based on the following specific feedback:\nFeedback:\n{sanitize_string(feedback)}\nCurrent Rust code:\n{sanitize_string(rust_code)}\nPlease strictly follow the steps mentioned in the prompt to optimize the code.\nEnsure all issues mentioned in the feedback are resolved, and add comments for each modification explaining the reason.\nOnly return the complete optimized Rust code without additional explanations.\n"""

            logging.info(f"修复专家prompt: {optimize_input}")
            optimized = optimize_agent.generate_response(optimize_input)
            logging.info(optimized)

            new_rust_code = extract_rust_code(optimized)
            if new_rust_code.strip():
                current_content = read_file_with_auto_encoding(rust_code_file)
                write_file_with_utf8(rust_code_file, current_content.replace(rust_code, new_rust_code))
                rust_code = new_rust_code
                logging.info(f"代码已针对静态分析进行优化 #{static_analysis_count}\n")
            else:
                logging.info("警告：代码优化专家没有返回有效的Rust代码。保持原代码不变。")

            continue  # 优化后重新进行静态分析
        else:
            logging.info("静态分析通过")
            c_to_rust_mappings[c_file]['items'] = rust_code
            break

    return rust_code


def process_files():
    parser = argparse.ArgumentParser(description='Process C files and convert them to Rust.')
    parser.add_argument('--c_code_dir', default='./Input/01-Primary', type=str, help='Directory path to analyze')
    parser.add_argument('--output_dir', default='./Output/primary', type=str, help='Directory path to save results')
    args = parser.parse_args()

    # 创建新的结果文件夹
    rust_code_dir = os.path.join(args.output_dir, "src")
    lib_file_path = os.path.join(rust_code_dir, "lib.rs")
    rust_test_dir = os.path.join(args.output_dir, "tests")

    # 清空lib.rs文件
    lib_rs_path = os.path.join(rust_code_dir, "lib.rs")
    if os.path.exists(lib_rs_path):
        with open(lib_rs_path, 'w', encoding='utf-8') as f:
            f.write("")

    successful_test_count = 0
    total_test_count = 0

    # translated_bytes keeps track of which portions of each file have already been translated
    translated_bytes = {}

    # Get dependencies and suggested translation order
    dependencies = analyze_directory(args.c_code_dir)
    translation_order = get_translation_order(dependencies)

    # Initialize dictionary to track C to Rust function and virable name mappings for each file
    c_to_rust_mappings = {os.path.splitext(os.path.basename(file))[0]: {} for file in translation_order}
    total_funcs = []
    for file in translation_order:
        c_funcs = extract_all_funcs(os.path.join(args.c_code_dir, file))
        total_funcs.extend(c_funcs)
        file_name = os.path.splitext(os.path.basename(file))[0]
        # 一些已经翻译过的变量、结构体或宏定义
        c_to_rust_mappings[file_name]['items'] = ''
        for func in c_funcs:
            c_to_rust_mappings[file_name][func] = ''

    logging.info(f"翻译顺序：{translation_order}")
    logging.info(f"翻译文件总数：{len(translation_order)}")
    
    # translation_order = ['test\\test-hash-functions.c'] # 调试
    for problem_path in translation_order:
        logging.info(f"开始翻译文件{problem_path}")
        if not problem_path.startswith("test"):
            continue

        problem_total_path = os.path.join(args.c_code_dir, problem_path)
        test_file_name = os.path.splitext(os.path.basename(problem_path))[0]
        rust_test_file_path = os.path.join(rust_test_dir, f"{test_file_name.replace('-', '_')}.rs")
        test_funcs = extract_test_functions(problem_total_path)

        for test_func in test_funcs:
            logging.info(f"开始翻译测试文件{test_file_name}的测试函数{test_func}")
            total_test_count += 1
            rust_codes = {}
            
            test_start_byte = 0
            if test_file_name in translated_bytes:
                test_start_byte = translated_bytes[test_file_name]
            test_funcs_codes, test_func_signatures, max_end_byte = code_decomposition(args.c_code_dir, test_file_name, test_func, test_start_byte)
            translated_bytes[test_file_name] = max_end_byte
            logging.info(f"需要翻译的函数的签名：{test_func_signatures}")

            logging.info(f"开始提取测试函数{test_func}的依赖文件和函数")
            depend_files_and_funcs = {}
            for test_func_signature in test_func_signatures:
                if not test_func_signature:
                    continue
                test_func_name = test_func_signature.split('(')[0].split(' ')[-1].replace('*', '').strip()
                # 递归函数，按顺序返回依赖文件中的函数
                depend_files_and_funcs[test_func_name] = extract_func_dependencies(args.c_code_dir, problem_path, test_func_name)

            # Merge all functions from depend_files_and_funcs into a single dict
            merged_funcs = {}
            for file_funcs in depend_files_and_funcs.values():
                if file_funcs:
                    for func_name, func_deps in file_funcs.items():
                        if func_name not in merged_funcs:
                            merged_funcs[func_name] = func_deps
                        else:
                            # Combine dependencies while preserving order and removing duplicates
                            merged_funcs[func_name] = list(dict.fromkeys(merged_funcs[func_name] + func_deps))
            
            depend_files_and_funcs = merged_funcs
            logging.info(f"依赖文件和函数：{depend_files_and_funcs}")

            for depend_file, depend_funcs in depend_files_and_funcs.items():
                logging.info(f"开始翻译依赖文件{depend_file}的函数{depend_funcs}之前的所有代码")
                rust_file_path = os.path.join(rust_code_dir, f"{depend_file.replace('-', '_')}.rs")

                rust_mod_name = os.path.splitext(os.path.basename(rust_file_path))[0]
                with open(lib_file_path, 'a', encoding='utf-8', errors='ignore') as file:
                    file.write(f"pub mod {rust_mod_name};\n")
                if not os.path.exists(rust_file_path):                    
                    with open(rust_file_path, 'w', encoding='utf-8') as f:
                        f.write("")

                if not depend_funcs:
                    continue
                
                # 对depend_file进行分割，再进行函数级翻译
                start_byte = 0
                if depend_file in translated_bytes:
                    start_byte = translated_bytes[depend_file]
                funcs_codes, func_signatures, max_end_byte = code_decomposition(args.c_code_dir, depend_file, depend_funcs, start_byte)
                translated_bytes[depend_file] = max_end_byte

                # 当前翻译文件依赖的文件
                depend_files = None
                for dependent_file, file_list in dependencies.items():
                    if depend_file == os.path.splitext(os.path.basename(dependent_file))[0]:
                        depend_files = file_list.copy()
                        break

                # 翻译文件的前置代码（变量、结构体、宏等）
                if start_byte == 0:
                    head_infos = head_info_extraction(args.c_code_dir, depend_file)
                    rust_code = convert_c_initialization_to_rust(
                    depend_file,
                    funcs_codes[0],
                    head_infos,
                    depend_files,
                    c_to_rust_mappings,
                    rust_file_path,
                    args.output_dir
                    )
                    func_signatures = func_signatures[1:]
                    funcs_codes = funcs_codes[1:]
                
                depend_files.append(depend_file)
                
                # 翻译函数级代码
                rust_code = convert_c_funcs_to_rust(
                    depend_file,
                    funcs_codes,
                    rust_file_path,
                    depend_files,
                    c_to_rust_mappings,
                    func_signatures,
                    total_funcs,
                    args.output_dir
                )
                rust_codes[depend_file.replace('-', '_')] = rust_code

            logging.info(f"测试文件{problem_path}的测试函数 {test_func} 的依赖项翻译完毕，开始翻译测试函数")
            
            depend_files = dependencies[problem_path]

            if test_start_byte == 0:
                # head_infos = head_info_extraction(args.c_code_dir, test_file_name)
                head_infos = {}
                rust_code = convert_c_initialization_to_rust(
                    test_file_name,
                    test_funcs_codes[0],
                    head_infos,
                    depend_files,
                    c_to_rust_mappings,
                    rust_test_file_path,
                    args.output_dir
                    )
                test_func_signatures = test_func_signatures[1:]
                test_funcs_codes = test_funcs_codes[1:]

            depend_files.append(test_file_name)

            rust_test = convert_c_funcs_to_rust(
                test_file_name,
                test_funcs_codes,
                rust_test_file_path,
                depend_files,
                c_to_rust_mappings,
                test_func_signatures,
                total_funcs,
                args.output_dir
            )
            rust_codes[test_file_name.replace('-', '_')] = rust_test

            logging.info(f"测试文件{problem_path}的测试函数 {test_func} 之前的所有代码的翻译完毕，开始测试")
            max_test_count = 5
            test_count = 0

            while test_count < max_test_count:
                test_count += 1
                test_success, dynamic_errors = compile_and_test_rust(rust_test, test_func, test_file_name.replace('-', '_'),  args.output_dir)
                if test_success and not dynamic_errors:
                    logging.info("编译和测试成功")
                    successful_test_count += 1
                    break
                
                logging.info(f"{test_func} 测试错误:\n {dynamic_errors}")

                # 编译失败或输出不匹配，进行优化
                logging.info("编译失败或者输出不匹配，继续优化")
                feedback_input = """\nAnalyze the following compilation error:\nIssue description:\n"""
                feedback_input += f"""{sanitize_string(dynamic_errors)}\nRust code:\n"""

                for depend_file_name, rust_code_segment in rust_codes.items():
                    feedback_input += f"""\n{sanitize_string(rust_code_segment)}\n########################################################"""
                    
                feedback_input += """\nPlease provide specific fix suggestions, but do not generate improved code."""
                logging.info(f"修复规划专家prompt: {feedback_input}")
                feedback = feedback_agent.generate_response(feedback_input)
                logging.info(feedback)

                optimize_input = """\nOptimize the Rust code based on the following specific feedback:\nFeedback:\n"""
                optimize_input += f"""{sanitize_string(feedback)}\nRust code:\n"""

                for i, rust_code_segment in enumerate(rust_codes.values()):
                    optimize_input += f"""\n###file {i+1}###\n{sanitize_string(rust_code_segment)}\n"""
                
                optimize_input += """\nPlease strictly follow the steps mentioned in the prompt to optimize the code.\nEnsure all issues mentioned in the feedback are resolved, and add comments for each modification explaining the reason.\nOnly return the complete optimized Rust code without additional explanations.
                """
                logging.info(f"修复专家prompt: {optimize_input}")
                optimized = optimize_agent_2.generate_response(optimize_input)
                logging.info(optimized)
                new_rust_code = optimized
                if new_rust_code.strip():

                    optimized_rust_codes = re.split(r'###file \d+###', new_rust_code)[1:]
                    for i, optimized_file_name in enumerate(rust_codes.keys()):
                        # Find the actual file path for optimized_file_name in output directory
                        for root, dirs, files in os.walk(args.output_dir):
                            if optimized_file_name+'.rs' in files:
                                optimized_file_path = os.path.join(root, optimized_file_name+'.rs')
                                current_content = read_file_with_auto_encoding(optimized_file_path)
                                write_file_with_utf8(optimized_file_path, current_content.replace(rust_codes[optimized_file_name].strip(), optimized_rust_codes[i].strip()))
                                rust_codes[optimized_file_name] = optimized_rust_codes[i]
                                break
                    logging.info(f"代码已针对输出不匹配进行优化 #{test_count}")
                else:
                    logging.info("警告：代码优化专家没有返回有效的Rust代码。保持原代码不变。")
    
    logging.info(f"测试文件总数：{total_test_count}")
    logging.info(f"测试成功数：{successful_test_count}")
    logging.info(f"测试成功率：{(successful_test_count/total_test_count * 100):.2f}%")


if __name__ == "__main__":
    setup_logging()
    process_files()