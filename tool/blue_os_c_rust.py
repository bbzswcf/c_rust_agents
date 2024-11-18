import os
import re
import sys
import time
import json
import chardet
import logging
import argparse
import subprocess
import unicodedata

from string import Template
from Agents import *
from input_prompt import *
from Agent_prompt import *
# from Agent_prompt_simple import *
from tree_sitter_analyzer import (
    analyze_directory,
    get_translation_order,
    extract_test_functions,
    sort_by_depend_count,
    dependencies_order
)

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
    return review_text


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

    # 使用 clippy 进行静态分析
    if "test" not in rust_file:
        insert_file_with_utf8(rust_file, "\nfn main() {}")
        clippy_result = subprocess.run(
            ["clippy-driver", rust_file],
            capture_output=True,
            text=True,
            encoding="utf-8"
        )
        # Remove the temporary main function
        remove_size = len("\nfn main() {}")
        original_size = os.path.getsize(rust_file)
        with open(rust_file, 'r+', encoding='utf-8') as f:
            f.truncate(original_size - remove_size)
    else:
        clippy_result = subprocess.run(
            ["cargo", "clippy", "--manifest-path", cargo_toml_dir],
            capture_output=True,
            text=True,
            encoding="utf-8"
        )

    if os.path.exists(os.path.splitext(os.path.basename(rust_file))[0]+".exe"):
        os.remove(os.path.splitext(os.path.basename(rust_file))[0]+".exe")
    if os.path.exists(os.path.splitext(os.path.basename(rust_file))[0]+".pdb"):
        os.remove(os.path.splitext(os.path.basename(rust_file))[0]+".pdb")

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


def compile_and_test_rust(test_func: str, temp_test_project_dir: str) -> tuple[bool, str]:
    current_dir = os.getcwd()
    cargo_toml = os.path.join(current_dir, os.path.join(temp_test_project_dir, "Cargo.toml"))

    try:
        compile_command = ["cargo", "test", test_func, "--manifest-path", cargo_toml]
        compile_result = subprocess.run(compile_command, capture_output=True, text=True, encoding="utf-8")
        logging.info("动态测试结果：")
        logging.info(compile_result.stdout+compile_result.stderr)

        issues = []
        compile_output = compile_result.stdout + compile_result.stderr
        error_blocks = re.findall(r'error.*?\n\n', compile_output, re.DOTALL)
        if not error_blocks:
            error_blocks = re.findall(r'failures:.*?\n\n\n', compile_output, re.DOTALL)
        for block in error_blocks:
            issues.append(block.strip())

        error_info = "\n\n".join(issues) if issues else ""

        if compile_result.returncode != 0:
            return False, f"Rust compile error:\n{error_info}"

        if "test result: ok" not in compile_result.stdout:
            return False, f"Rust test error:\n{error_info}"

        return True, ""
    except Exception as e:
        return False, f"Rust test error:\n{str(e)}"


def convert_c_funcs_to_rust(c_file: str, func_name: str, c_func_info: dict, rust_items: str, rust_code_file: str,
                             rust_result_dir: str, metadata: dict) -> tuple[bool, str]:
    c_code = c_func_info['code']
    
    # logging.info("开始API转换")
    # api_conversion = api_agent.generate_response(
    #     f"""\nExtract and convert only the C-specific APIs to their Rust equivalents:\n{c_code}\n"""
    # )
    # logging.info(api_conversion)
    logging.info("开始语法转换")

    # 从c_func_info中找到对应depend_file中的Rust_func_name
    c_signatures = []
    rust_signatures = []
    for depend_func_file in c_func_info['depend_funcs']:
        for func_info_metadata in metadata[depend_func_file['file']]['functions']:
            if func_info_metadata['name'] == depend_func_file['name']:
                c_signatures.append(func_info_metadata['signature'])
                rust_signatures.append(func_info_metadata['rust_signature'])
                break
    function_call_mappings = ""
    for c_func, rust_func in zip(c_signatures, rust_signatures):
        if rust_func:
            function_call_mappings += f"""{c_func} <----> {rust_func}\n"""

    template = Template(Syntax_prompt_2)
    combined_syntax_input = template.substitute(c_code=c_code, rust_items=rust_items, function_call_mappings=function_call_mappings)
    # combined_syntax_input += """\nOnly return the function implementation without redefining any structs, variables, or types that are already defined in the codebase."""
    logging.info(f"语法专家prompt: {combined_syntax_input}")

    rust_code = syntax_agent.generate_response(combined_syntax_input, Syntax_example_input, Syntax_example_output)
    logging.info(rust_code)
    rust_code = extract_rust_code(rust_code)

    original_size = os.path.getsize(rust_code_file)
    # Add #[test] attribute if this is a test function
    if "test_" in func_name:
        rust_code = "#[test]\n" + rust_code
    insert_file_with_utf8(rust_code_file, rust_code)

    max_static_analysis_and_test_attempts = 4
    static_analysis_count = 0
    correct = False

    while static_analysis_count < max_static_analysis_and_test_attempts:
        static_analysis_count += 1
        logging.info(f"静态分析尝试 #{static_analysis_count}")
        analysis_result = static_analysis(rust_result_dir, rust_code_file)

        if analysis_result:
            logging.info("发现静态分析问题，正在优化...")
            feedback_input = feedback_input_prompt.format(c_code=c_code, rust_code=rust_code, error_msg=sanitize_string(analysis_result))

            logging.info(f"修复规划专家prompt: {feedback_input}")
            feedback = feedback_agent.generate_response(feedback_input)
            logging.info(feedback)

            optimize_input = f"""\nOptimize the Rust code based on the following specific feedback:\nFeedback:\n{sanitize_string(feedback)}\nCurrent Rust code:\n{sanitize_string(rust_code)}\nPlease strictly follow the steps mentioned in the prompt to optimize the code.\nEnsure all issues mentioned in the feedback are resolved, and add comments for each modification explaining the reason.\nOnly return the complete optimized Rust code without additional explanations.\n"""

            logging.info(f"修复专家prompt: {optimize_input}")
            optimized = optimize_agent.generate_response(optimize_input)
            logging.info(optimized)

            new_rust_code = extract_rust_code(optimized)
            if new_rust_code.strip():
                with open(rust_code_file, 'r+') as f:
                    f.truncate(original_size)
                insert_file_with_utf8(rust_code_file, new_rust_code)
                rust_code = new_rust_code
                logging.info(f"代码已针对静态分析进行优化 #{static_analysis_count}\n")
            else:
                logging.info("警告：代码优化专家没有返回有效的Rust代码。保持原代码不变。")
        else:
            logging.info("静态分析通过")
            correct = True
            break

    if not correct:
        analysis_result = static_analysis(rust_result_dir, rust_code_file)
        if not analysis_result:
            logging.info("静态分析通过")
            correct = True

    with open(rust_code_file, 'r+') as f:
        f.truncate(original_size)
    return correct, rust_code


def convert_c_initialization_to_rust(c_file: str, rust_code_file: str, rust_result_dir: str, metadata: dict) -> str:
    pre_code = ''
    for head_code in metadata[c_file]['head_info']:
        pre_code = pre_code + head_code['code'].strip() + '\n\n'
    for variable in metadata[c_file]['variables']:
        pre_code = pre_code + variable['code'].strip() + '\n\n'
    pre_code = pre_code.strip()
    if pre_code == '':
        return ''

    logging.info("开始语法转换")
    if 'test' in c_file:
        combined_syntax_input = f"""\nConvert the following C code to Rust.\nC code:\n{pre_code}\nOutput only the converted Rust code without any explanations.\n"""
    else:
        combined_syntax_input = type_convert_input_prompt.format(c_code=pre_code)
    logging.info(f"语法专家prompt: {combined_syntax_input}")

    rust_code = syntax_agent.generate_response(combined_syntax_input)
    logging.info(rust_code)
    rust_code = extract_rust_code(rust_code)
    original_size = os.path.getsize(rust_code_file)
    insert_file_with_utf8(rust_code_file, rust_code)

    max_static_analysis_and_test_attempts = 4
    static_analysis_count = 0

    while static_analysis_count < max_static_analysis_and_test_attempts:
        static_analysis_count += 1
        logging.info(f"静态分析尝试 #{static_analysis_count}")
        analysis_result = static_analysis(rust_result_dir, rust_code_file)

        if analysis_result:
            logging.info("发现静态分析问题，正在优化...")
            feedback_input = feedback_input_prompt.format(c_code=pre_code, rust_code=rust_code, error_msg=sanitize_string(analysis_result))

            logging.info(f"修复规划专家prompt: {feedback_input}")
            feedback = feedback_agent.generate_response(feedback_input)
            logging.info(feedback)

            optimize_input = f"""\nOptimize the Rust code based on the following specific feedback:\nFeedback:\n{sanitize_string(feedback)}\nCurrent Rust code:\n{sanitize_string(rust_code)}\nPlease strictly follow the steps mentioned in the prompt to optimize the code.\nEnsure all issues mentioned in the feedback are resolved, and add comments for each modification explaining the reason.\nOnly return the complete optimized Rust code without additional explanations.\n"""

            logging.info(f"修复专家prompt: {optimize_input}")
            optimized = optimize_agent.generate_response(optimize_input)
            logging.info(optimized)

            new_rust_code = extract_rust_code(optimized)
            if new_rust_code.strip():
                with open(rust_code_file, 'r+') as f:
                    f.truncate(original_size)
                insert_file_with_utf8(rust_code_file, new_rust_code)
                rust_code = new_rust_code
                logging.info(f"代码已针对静态分析进行优化 #{static_analysis_count}\n")
            else:
                logging.info("警告：代码优化专家没有返回有效的Rust代码。保持原代码不变。")

            continue  # 优化后重新进行静态分析
        else:
            logging.info("静态分析通过")
            break
    
    with open(rust_code_file, 'r+') as f:
        f.truncate(original_size)
    return rust_code


def process_files():
    parser = argparse.ArgumentParser(description='Process C files and convert them to Rust.')
    parser.add_argument('--c_code_dir', default='./Input/01-Primary', type=str, help='Directory path to analyze')
    parser.add_argument('--output_dir', default='./Output/primary', type=str, help='Directory path to save results')
    args = parser.parse_args()

    # 创建新的结果文件夹
    rust_code_dir = os.path.join(args.output_dir, "src")
    lib_file_path = os.path.join(rust_code_dir, "lib.rs")
    # rust_test_dir = os.path.join(args.output_dir, "tests")

    successful_test_count = 0
    total_test_count = 0

    # 读取元数据
    metadata = json.load(open('tool/c_metadata.json', 'r', encoding='utf-8'))

    # translated_flags keeps track of which functions of each file have already been translated
    translated_flags = {}
    for file_relapath, file_info in metadata.items():
        translated_flags[file_relapath] = {}
        for func_info in file_info['functions']:
            translated_flags[file_relapath][func_info['name']] = None
    # 调试
    # translated_flags['src\\arraylist.c']['arraylist_new'] = True
    # translated_flags['src\\arraylist.c']['arraylist_free'] = True
    # translated_flags['src\\arraylist.c']['arraylist_enlarge'] = True
    # translated_flags['src\\arraylist.c']['arraylist_insert'] = True
    # translated_flags['src\\arraylist.c']['arraylist_append'] = True
    # translated_flags['src\\arraylist.c']['arraylist_prepend'] = True
    # translated_flags['src\\arraylist.c']['arraylist_clear'] = True
    # translated_flags['src\\arraylist.c']['arraylist_index_of'] = True
    # translated_flags['src\\arraylist.c']['arraylist_sort_internal'] = True
    # translated_flags['src\\arraylist.c']['arraylist_sort'] = True
    # translated_flags['src\\compare-int.c']['int_compare'] = True


    
    dependencies = analyze_directory(metadata)
    translation_order = get_translation_order(dependencies)

    logging.info(f"翻译顺序：{translation_order}")
    logging.info(f"翻译文件总数：{len(translation_order)}")
    
    # translation_order = translation_order[14:24] # 调试 compare 10/41
    # translation_order = translation_order[:14] # 调试 arraylist 3/30
    # translation_order = translation_order[24:30] # 调试 queue 11/34
    # translation_order = translation_order[30:32] # 调试 test-cpp
    translation_order = translation_order[32:34] # 调试 test-rb-tree
    # translation_order = translation_order[34:] # 调试 test-sortedarray
    for problem_path in translation_order:
        logging.info(f"开始翻译文件{problem_path}")
        if not problem_path.startswith("test"):
            continue

        problem_total_path = os.path.join(args.c_code_dir, problem_path)
        test_file_name = os.path.splitext(os.path.basename(problem_path))[0]
        rust_test_file_path = os.path.join(rust_code_dir, f"{test_file_name.replace('-', '_')}.rs")

        # 获取所有c文件名
        c_file_relapaths = list(metadata.keys())
        c_file_names = [os.path.splitext(os.path.basename(c_file_relapath))[0] for c_file_relapath in c_file_relapaths]

        # 测试函数按依赖函数数量升序
        test_funcs = extract_test_functions(problem_path, metadata)
        test_funcs = sort_by_depend_count(test_funcs, problem_path, metadata)

        # 构建测试环境
        depend_file_list = []
        for func_info in metadata[problem_path]['functions']:
            for depend_func in func_info['depend_funcs']:
                if depend_func['file'] not in depend_file_list and depend_func['file'].startswith("src"):
                    depend_file_list.append(depend_func['file'])
        
        write_file_with_utf8(lib_file_path, f"pub(crate) mod utils;\npub(crate) mod {test_file_name.replace('-', '_')};\n")
        for depend_file in depend_file_list:
            rust_mod_name = os.path.splitext(os.path.basename(depend_file.replace('-', '_')))[0]
            write_file_with_utf8(os.path.join(rust_code_dir, f"{rust_mod_name}.rs"), "")            
            insert_file_with_utf8(lib_file_path, f"pub(crate) mod {rust_mod_name};\n")
        write_file_with_utf8(rust_test_file_path, "")

        for test_func in test_funcs:
            logging.info(f"开始翻译测试文件{test_file_name}的测试函数{test_func}")
            total_test_count += 1            

            logging.info(f"开始提取测试函数{test_func}的依赖文件和函数")
            depend_files_and_funcs = dependencies_order(test_func, problem_path, metadata)
            logging.info(f"依赖文件和函数：{depend_files_and_funcs}")

            # 如果之前翻译过某一依赖函数，且翻译失败了，直接跳过当前测试函数的翻译
            flag = False
            for depend_func, depend_file in depend_files_and_funcs:
                if translated_flags[depend_file][depend_func] == False:
                    logging.info(f"依赖文件{depend_file}的函数{depend_func}之前翻译失败，跳过当前测试函数{test_func}的翻译")
                    flag = True
                    break
            if flag:
                continue
            
            # 清空依赖文件对应的rust文件
            write_file_with_utf8(rust_test_file_path, "")
            for depend_func, depend_file in depend_files_and_funcs:
                depend_file_name = os.path.splitext(os.path.basename(depend_file.replace('-', '_')))[0]
                rust_file_path = os.path.join(rust_code_dir, f"{depend_file_name}.rs") 
                write_file_with_utf8(rust_file_path, "")

            # 标记是否有依赖函数翻译错误
            has_translation_error = False
            for depend_func, depend_file in depend_files_and_funcs:
                logging.info(f"开始翻译依赖文件{depend_file}的函数{depend_func}")
                depend_file_name = os.path.splitext(os.path.basename(depend_file.replace('-', '_')))[0]
                rust_file_path = os.path.join(rust_code_dir, f"{depend_file_name}.rs")
                
                # 构建测试环境
                content = read_file_with_auto_encoding(rust_file_path)
                if not content.strip():
                    insert_file_with_utf8(rust_file_path, f"use crate::utils::*;\n")
                    for include in metadata[depend_file]['includes']:
                        match = re.search(r'#include\s*[<"]([^>"]+)[>"]', include['code'])
                        header_name = match.group(1)
                        header_base = os.path.splitext(header_name)[0]
                        if header_base in c_file_names and header_base != os.path.splitext(os.path.basename(depend_file))[0]:
                            insert_file_with_utf8(rust_file_path, f"use crate::{header_base.replace('-', '_')}::*;\n")
                    # 翻译文件的前置代码（变量、结构体、宏等）
                    if not metadata[depend_file]['rust_items'].strip():
                        metadata[depend_file]['rust_items'] = convert_c_initialization_to_rust(
                            depend_file,
                            rust_file_path,
                            args.output_dir,
                            metadata
                        )
                    insert_file_with_utf8(rust_file_path, metadata[depend_file]['rust_items'])

                # 如果之前翻译过该函数，则直接插入翻译结果
                if translated_flags[depend_file][depend_func] == True:
                    logging.info(f"依赖文件{depend_file}的函数{depend_func}之前翻译成功，无需再次翻译")
                    for func in metadata[depend_file]['functions']:
                        if func['name'] == depend_func:
                            insert_file_with_utf8(rust_file_path, func['rust_code'])
                    continue
                
                # 翻译函数级代码
                c_func_info = None
                for func_info_metadata in metadata[depend_file]['functions']:
                    if func_info_metadata['name'] == depend_func:
                        c_func_info = func_info_metadata
                        break

                rust_items = ''
                include_files = []
                for include in metadata[depend_file]['includes']:
                    match = re.search(r'#include\s*[<"]([^>"]+)[>"]', include['code'])
                    if match:
                        header_name = match.group(1)
                        include_files.append(os.path.splitext(header_name)[0])
                for include_file in include_files:
                    if include_file in c_file_names and 'test' not in include_file:
                        include_file = 'src\\' + include_file + '.c'
                        rust_items += metadata[include_file]['rust_items'] + '\n'
                rust_items += metadata[depend_file]['rust_items'] + '\n'
                rust_items = rust_items.strip()
                success,rust_code = convert_c_funcs_to_rust(
                    depend_file,
                    depend_func,
                    c_func_info,
                    rust_items,
                    rust_file_path,
                    args.output_dir,
                    metadata
                )
                if not success:
                    translated_flags[depend_file][depend_func] = False
                    has_translation_error = True
                    logging.info(f"依赖文件{depend_file}的函数{depend_func}翻译失败")
                    break
                rust_signatures = re.findall(r'fn\s*([\s\S]*?){', rust_code)
                if rust_signatures:
                    c_func_info['rust_signature'] = rust_signatures[0]
                c_func_info['rust_code'] = rust_code
                insert_file_with_utf8(rust_file_path, rust_code)

            if has_translation_error:
                logging.info(f"测试文件{problem_path}的测试函数 {test_func} 的依赖项翻译失败，跳过当前测试函数")
                continue
            
            logging.info(f"测试文件{problem_path}的测试函数 {test_func} 的依赖项翻译完毕，开始翻译测试函数")
            
            # 构建测试文件的测试环境
            test_func_info = None
            for func_info in metadata[problem_path]['functions']:
                if func_info['name'] == test_func:
                    test_func_info = func_info
                    break
            for depend_func in test_func_info['depend_funcs']:
                if depend_func['file'] != problem_path:
                    head_file_info = os.path.splitext(os.path.basename(depend_func['file'].replace('-', '_')))[0]
                    current_content = read_file_with_auto_encoding(rust_test_file_path)
                    if f"use crate::{head_file_info}::*;" not in current_content:
                        insert_file_with_utf8(rust_test_file_path, f"use crate::{head_file_info}::*;\n")
            if "use crate::utils::*;" not in current_content:
                insert_file_with_utf8(rust_test_file_path, f"use crate::utils::*;\n")

            if not metadata[problem_path]['rust_items'].strip():
                rust_code = convert_c_initialization_to_rust(
                    problem_path,
                    rust_test_file_path,
                    args.output_dir,
                    metadata
                )
                metadata[problem_path]['rust_items'] = rust_code
            current_content = read_file_with_auto_encoding(rust_test_file_path)
            if metadata[problem_path]['rust_items'] not in current_content:
                insert_file_with_utf8(rust_test_file_path, metadata[problem_path]['rust_items'])
                
            rust_items = ''
            for depend_file in depend_file_list:
                rust_items += metadata[depend_file]['rust_items'] + '\n'
            rust_items += metadata[problem_path]['rust_items'] + '\n'
            rust_items = rust_items.strip()
            success,rust_test = convert_c_funcs_to_rust(
                test_file_name,
                test_func,
                test_func_info,
                rust_items,
                rust_test_file_path,
                args.output_dir,
                metadata
            )
            rust_signatures = re.findall(r'fn\s*([\s\S]*?){', rust_test)
            if rust_signatures:
                test_func_info['rust_signature'] = rust_signatures[0]
            test_func_info['rust_code'] = rust_test
            insert_file_with_utf8(rust_test_file_path, rust_test)

            logging.info(f"测试文件{problem_path}的测试函数 {test_func} 之前的所有代码的翻译完毕，开始测试")
            max_test_count = 4
            test_count = 0

            #将所有代码写入一个文件进行测试
            temp_test_project_dir = args.output_dir+'_temp'
            temp_test_src_dir = os.path.join(temp_test_project_dir, 'src')
            temp_test_lib_path = os.path.join(temp_test_src_dir, 'lib.rs')
            temp_test_file_path = os.path.join(temp_test_src_dir, 'temp.rs')

            write_file_with_utf8(temp_test_lib_path, "pub(crate) mod utils;\npub(crate) mod temp;\n")

            depend_files = []
            for _, depend_file in depend_files_and_funcs:
                if depend_file not in depend_files:
                    depend_files.append(depend_file)

            # 将rust代码整合在一起，同时整合c代码
            total_rust_code = ''
            total_c_code = ''
            append_flag = True
            for depend_file in depend_files:
                if depend_file == problem_path:
                    append_flag = False
                total_rust_code += metadata[depend_file]['rust_items'] + '\n\n'
                total_c_code += '\n'.join(metadata[depend_file]['head_info']) + '\n'
                total_c_code += '\n'.join(metadata[depend_file]['variables']) + '\n'
            if append_flag:
                total_c_code += '\n'.join(metadata[problem_path]['head_info']) + '\n'
                total_c_code += '\n'.join(metadata[problem_path]['variables']) + '\n'
                total_rust_code += metadata[problem_path]['rust_items'] + '\n\n'
            for depend_func, depend_file in depend_files_and_funcs:
                for func_info in metadata[depend_file]['functions']:
                    if func_info['name'] == depend_func:
                        total_rust_code += func_info['rust_code'] + '\n\n'
                        total_c_code += func_info['code'] + '\n'
            total_rust_code += test_func_info['rust_code']
            total_c_code += test_func_info['code'] + '\n'

            total_rust_code = "use crate::utils::*;\n" + total_rust_code
            write_file_with_utf8(temp_test_file_path, total_rust_code)

            success_flag = False
            while test_count < max_test_count:
                test_count += 1
                test_success, dynamic_errors = compile_and_test_rust(test_func, temp_test_project_dir)
                if test_success and not dynamic_errors:
                    logging.info(f"测试文件{problem_path}的测试函数 {test_func} 编译和测试成功")
                    success_flag = True
                    # todo: 更新translated_flags
                    for depend_func, depend_file in depend_files_and_funcs:
                        translated_flags[depend_file][depend_func] = True
                    translated_flags[problem_path][test_func] = True
                    successful_test_count += 1
                    break
                
                logging.info(f"{test_func} 测试错误\n")

                # 运行失败，进行优化
                logging.info("运行失败，继续优化")
                failed_rust_code = read_file_with_auto_encoding(temp_test_file_path)
                feedback_input = feedback_input_prompt.format(c_code=total_c_code, rust_code=failed_rust_code, error_msg=sanitize_string(dynamic_errors))
                logging.info(f"修复规划专家prompt: {feedback_input}")
                feedback = feedback_agent.generate_response(feedback_input)
                logging.info(feedback)

                rust_context = ''
                for depend_file in depend_files:
                    rust_context += metadata[depend_file]['rust_items'] + "\n"
                rust_context += metadata[problem_path]['rust_items'] + "\n"
                rust_functions = ''
                for i, (depend_func, depend_file) in enumerate(depend_files_and_funcs):
                    for func_info in metadata[depend_file]['functions']:
                        if func_info['name'] == depend_func:
                            rust_functions += f"""\n###function {i+1}###\n{sanitize_string(func_info['rust_code'])}\n"""
                rust_functions += f"""\n###function {i+2}###\n{sanitize_string(test_func_info['rust_code'])}\n"""

                optimize_input = optimize_input_prompt.format(feedback=sanitize_string(feedback), rust_items=rust_context, functions=rust_functions)
                logging.info(f"修复专家prompt: {optimize_input}")
                optimized = optimize_agent_2.generate_response(optimize_input)
                logging.info(optimized)
                new_rust_code = extract_rust_code(optimized)
                if new_rust_code.strip():
                    optimized_rust_codes = re.split(r'###function \d+###', new_rust_code)[1:]
                    optimized_rust_codes = [code.strip() for code in optimized_rust_codes]
                    if len(optimized_rust_codes) != len(depend_files_and_funcs) + 1:
                        logging.info("警告：代码优化专家没有返回正确的函数数量。保持原代码不变。")
                        continue
                    
                    for i, (depend_func, depend_file) in enumerate(depend_files_and_funcs):
                        for func_info in metadata[depend_file]['functions']:
                            if func_info['name'] == depend_func and translated_flags[depend_file][depend_func] == None:
                                func_info['rust_code'] = optimized_rust_codes[i]
                                optimized_rust_signatures = re.findall(r'fn\s*([\s\S]*?){', optimized_rust_codes[i])
                                if optimized_rust_signatures:
                                    func_info['rust_signature'] = optimized_rust_signatures[0]
                                break
                    test_func_info['rust_code'] = optimized_rust_codes[-1]
                    test_func_rust_signatures = re.findall(r'fn\s*([\s\S]*?){', optimized_rust_codes[-1])
                    if test_func_rust_signatures:
                        test_func_info['rust_signature'] = test_func_rust_signatures[0]
                    optimized_rust_code = ''
                    append_flag = True
                    for depend_file in depend_files:
                        if depend_file == problem_path:
                            append_flag = False
                        optimized_rust_code += metadata[depend_file]['rust_items'] + "\n"
                    if append_flag:
                        optimized_rust_code += metadata[problem_path]['rust_items'] + "\n"
                    optimized_rust_code += "\n\n".join(optimized_rust_codes)
                    optimized_rust_code = "use crate::utils::*;\n" + optimized_rust_code
                    write_file_with_utf8(temp_test_file_path, optimized_rust_code)
                    
                    logging.info(f"代码已针对运行失败进行优化 #{test_count}")
                else:
                    logging.info("警告：代码优化专家没有返回有效的Rust代码。保持原代码不变。")
    
            if not success_flag:
                # Update translated_flags to mark test function as failed
                translated_flags[problem_path][test_func_info['name']] = False
                for depend_func, depend_file in depend_files_and_funcs:
                    if translated_flags[depend_file][depend_func] == None:
                        translated_flags[depend_file][depend_func] = False
                logging.info(f"测试函数 {test_func_info['name']} 翻译失败")

    # Write back metadata to file
    with open('tool/c_metadata.json', 'w', encoding='utf-8') as f:
        json.dump(metadata, f, indent=2, ensure_ascii=False)

    logging.info(f"测试函数总数：{total_test_count}")
    logging.info(f"测试成功数：{successful_test_count}")
    logging.info(f"测试成功率：{(successful_test_count/total_test_count * 100):.2f}%")


if __name__ == "__main__":
    setup_logging()
    process_files()