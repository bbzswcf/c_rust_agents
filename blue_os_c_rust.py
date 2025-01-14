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
import shutil
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
    dependencies_order,
    extract_rust_funcs
)
from c_code_preprocess import code_preprocess

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
    if review_text is None or review_text == "":
        logging.info("警告：从该专家收到空响应")
        return ""
    code_blocks = re.findall(r'```rust\n(.*?)```', review_text, re.DOTALL)
    if code_blocks:
        review_text = code_blocks[0].strip()
        # 移除重复的 translation_utils 导入
        review_text = re.sub(r'use\s+translation_utils::\{[^}]+\};\s*', '', review_text)
        review_text = re.sub(r'use\s+crate::translation_utils::\{[^}]+\};\s*', '', review_text)
    review_text = re.sub(r'//.*$', '', review_text, flags=re.MULTILINE)
    return review_text.strip()


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
    
    with open(rust_file) as file:
        rust_file_content = file.readlines()
    logging.info(f"静态分析内容：\n{''.join(rust_file_content)}")
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

    # logging.info("静态分析结果：")
    # logging.info(clippy_result.stdout+clippy_result.stderr)
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
        # 添加 --quiet 参数来抑制警告,只显示错误
        # 添加 RUSTFLAGS 环境变量来设置编译标志
        env = os.environ.copy()
        env['RUSTFLAGS'] = '-A warnings'  # -A warnings 表示允许所有警告
        
        compile_command = [
            "cargo", "test", 
            test_func, 
            "--manifest-path", cargo_toml,
            "--quiet"  # 只显示错误信息
        ]
        compile_result = subprocess.run(
            compile_command, 
            capture_output=True, 
            text=True, 
            encoding="utf-8",
            env=env  # 使用修改后的环境变量
        )
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
                if line_number not in rust_lines_map or original_line.strip() != rust_lines_map[line_number].strip():
                    accept = False
                    break
            if accept:
                applied_logpairs.append((original_code, fixed_code))
    return applied_logpairs

def extract_macro_definitions(rust_code: str) -> list[str]:
    """
    从Rust代码中提取宏定义的名称
    """
    if not rust_code:
        return []
    # 查找所有的宏定义
    macro_pattern = r'macro_rules!\s+(\w+)'
    macros = re.findall(macro_pattern, rust_code)
    logging.info(f"从代码中提取到的宏: {macros}")
    return macros

def find_similar_rust_code(rust_code: str) -> str:
    try:
        from rag_openai import OpenAICustomEmbeddings, gpt4_key, gpt_base_url
        from langchain_community.vectorstores import Chroma
        
        embeddings = OpenAICustomEmbeddings(
            api_key=gpt4_key,
            base_url=gpt_base_url
        )
        
        vectorstore = Chroma(
            persist_directory="./rag_db",
            embedding_function=embeddings,
        )
        
        # 获取更多结果并按相似度排序
        results = vectorstore.similarity_search_with_relevance_scores(rust_code, k=50)
        sorted_results = sorted(results, key=lambda x: x[1], reverse=True)
        
        if sorted_results:
            # 打印所有结果的相似度分数
            for i, (doc, score) in enumerate(sorted_results[:2], 1):  # 只打印前2个结果用于调试
                logging.info(f"结果 {i}:")
                logging.info(f"相似度分数: {score}")
                logging.info(f"代码内容:\n{doc.page_content}\n")
                logging.info("-" * 50)
            
            # 返回相似度最高的结果
            return sorted_results[0][0].page_content.replace('\\n', '\n')
            
    except Exception as e:
        logging.error(f"搜索相似代码时出错: {str(e)}")
    
    return ""

def find_similar_rust_test_code(rust_code: str) -> str:
    """
    在知识库中搜索与给定Rust代码最相似的示例。
    Returns:
        str: 最相似的Rust代码，如果未找到则返回空字符串
    """
    try:
        from rag_openai import OpenAICustomEmbeddings, gpt4_key, gpt_base_url
        from langchain_community.vectorstores import Chroma
        
        embeddings = OpenAICustomEmbeddings(
            api_key=gpt4_key,
            base_url=gpt_base_url
        )
        
        vectorstore = Chroma(
            persist_directory="./rag_db_test",
            embedding_function=embeddings,
            collection_metadata={
                "hnsw:space": "cosine",         
                "hnsw:ef_search": 1000,         
                "hnsw:ef_construction": 2000,   
                "hnsw:M": 128                   
            }
        )
        
        # 获取更多结果并按相似度排序
        results = vectorstore.similarity_search_with_relevance_scores(rust_code, k=50)
        sorted_results = sorted(results, key=lambda x: x[1], reverse=True)
        
        if sorted_results:
            # 打印所有结果的相似度分数
            for i, (doc, score) in enumerate(sorted_results[:2], 1):  # 只打印前5个结果用于调试
                logging.info(f"结果 {i}:")
                logging.info(f"相似度分数: {score}")
                logging.info(f"代码内容:\n{doc.page_content}\n")
                logging.info("-" * 50)
            
            # 返回相似度最高的结果
            return sorted_results[0][0].page_content.replace('\\n', '\n')
            
    except Exception as e:
        logging.error(f"搜索相似代码时出错: {str(e)}")
    return ""

def convert_c_funcs_to_rust(c_file: str, func_name: str, c_func_info: dict, rust_items: str, rust_code_file: str,
                             rust_result_dir: str, metadata: dict, user_input: str, example_input: list[str], example_output: list[str]) -> tuple[bool, str]:
    # 添加对红黑树case函数的特殊处理
    if 'rb-tree.c' in c_file and 'rb_tree_insert_case' in func_name:
        logging.info("检测到红黑树case函数，进行批量处理")
        
        # 收集所有case函数
        case_funcs = {}
        for func_info in metadata[c_file]['functions']:
            if 'rb_tree_insert_case' in func_info['name']:
                case_funcs[func_info['name']] = func_info
        
        # 单独翻译每个case函数
        all_rust_code = []
        for case_name, case_func in case_funcs.items():
            logging.info(f"\n开始翻译函数: {case_name}")
            
            # 如果已经有翻译结果，直接使用
            if 'rust_code' in case_func and case_func['rust_code']:
                logging.info(f"函数 {case_name} 已有翻译结果，跳过翻译")
                all_rust_code.append(case_func['rust_code'])
                continue
                
            c_code = case_func['code']
            
            # 从c_func_info中找到对应depend_file中的Rust_func_name
            c_signatures = []
            rust_signatures = []
            for depend_func_file in case_func['depend_funcs']:
                for func_info_metadata in metadata[depend_func_file['file']]['functions']:
                    if func_info_metadata['name'] == depend_func_file['name']:
                        c_signatures.append(func_info_metadata['signature'])
                        rust_signatures.append(func_info_metadata['rust_signature'])
                        break
            function_call_mappings = ""
            for c_func, rust_func in zip(c_signatures, rust_signatures):
                if rust_func:
                    function_call_mappings += f"""{c_func} <----> {rust_func}\n"""

            # 根据是否是测试函数选择不同的搜索函数
            if "test_" in case_name:
                similar_rust_code = find_similar_rust_test_code(c_code)
                logging.info("使用测试代码库进行搜索")
            else:
                similar_rust_code = find_similar_rust_code(c_code)
                logging.info("使用普通代码库进行搜索")
                
            if not similar_rust_code:
                logging.info(f"未找到函数 {case_name} 的相似Rust代码参考")
                continue
                
            logging.info(f"\n找到相似的Rust代码参考:\n{similar_rust_code}")
            
            # 使用原有的语法专家进行转换
            template = Template(user_input)
            combined_syntax_input = template.substitute(c_code=c_code, similar_rust_code=similar_rust_code, rust_items=rust_items, function_call_mappings=function_call_mappings)
            logging.info(f"语法专家prompt: {combined_syntax_input}")
            
            rust_code = syntax_agent.generate_response(combined_syntax_input, example_input, example_output)
            logging.info("语法专家输出：")
            logging.info(rust_code)
            
            rust_code = extract_rust_code(rust_code)
            all_rust_code.append(rust_code)
            case_func['rust_code'] = rust_code
            
            # 提取rust签名
            rust_signatures = re.findall(r'fn\s*([\s\S]*?){', rust_code)
            if rust_signatures:
                case_func['rust_signature'] = rust_signatures[0]
            
        # 合并所有翻译后的代码进行一次性静态分析
        if all_rust_code:
            combined_rust_code = '\n\n'.join(all_rust_code)
            logging.info(f"\n合并后的所有case函数代码:\n{combined_rust_code}")
            
            # 读取当前文件内容
            current_content = read_file_with_auto_encoding(rust_code_file)
            
            # 检查每个case函数是否已存在于文件中
            existing_funcs = set()
            for case_name in case_funcs.keys():
                if f"fn {case_name}<" in current_content:
                    existing_funcs.add(case_name)
                    logging.info(f"函数 {case_name} 已存在于文件中")
            
            # 只保留不存在的函数进行静态分析
            new_rust_code = []
            for code in all_rust_code:
                func_name_match = re.search(r'fn\s+(\w+)<', code)
                if func_name_match and func_name_match.group(1) not in existing_funcs:
                    new_rust_code.append(code)
            
            if not new_rust_code:
                logging.info("所有函数都已存在，无需进行静态分析")
                return True, case_funcs[func_name]['rust_code']
            
            # 进行一次性静态分析
            original_size = os.path.getsize(rust_code_file)
            original_line_num = 0
            with open(rust_code_file) as f:
                original_line_num = len(f.readlines())
            insert_file_with_utf8(rust_code_file, '\n\n'.join(new_rust_code))
            
            max_static_analysis_and_test_attempts = 2
            static_analysis_count = 0
            correct = False

            while static_analysis_count < max_static_analysis_and_test_attempts:
                static_analysis_count += 1
                logging.info(f"静态分析尝试 #{static_analysis_count}")
                analysis_result = static_analysis(rust_result_dir, rust_code_file)

                if analysis_result:
                    logging.info("发现静态分析问题，正在优化...")
                    feedback_input = feedback_input_prompt.format(c_code=c_code, rust_code='\n\n'.join(new_rust_code), error_msg=sanitize_string(analysis_result))

                    logging.info(f"修复规划专家prompt: {feedback_input}")
                    feedback = feedback_agent.generate_response(feedback_input)
                    logging.info("修复规划专家输出：")
                    logging.info(feedback)

                    similar_rust_code = find_similar_rust_code('\n\n'.join(new_rust_code))
                    if similar_rust_code:
                        logging.info("找到相似的Rust代码参考:")
                        logging.info(similar_rust_code)

                    optimize_input = (
                        "Optimize the Rust code based on the feedback and provided reference code.\n"
                        "Reference code:\n"
                        "{}"
                        "Feedback to address:\n"
                        "{}"
                        "Code to optimize:\n"
                        "{}"
                        "\nAvailable types and utilities in translation_utils:\n"
                        "1. Pointer Types:\n"
                        "   - Vector<T>, Manual<T>, Owned<T>, Unowned<T>, SpanView<T>, Ptr<T>\n"
                        "2. Memory Operations:\n"
                        "   - c_malloc!, c_free!, c_calloc!, c_realloc!\n"
                        "   - c_memcpy!, c_memmove!, c_memset!\n"
                        "   - c_sizeof!, c_ref!\n"
                        "3. String Operations:\n"
                        "   - CStr type\n"
                        "   - c_strlen!, c_strcmp!, c_strdup!, c_sprintf!\n"
                        "   - c_tolower!, c_toupper!\n"
                        "4. Control Flow:\n"
                        "   - c_for!\n"
                        "Type Conversion Rules:\n"
                        "1. char* -> CStr\n"
                        '2. char array[] = "string" -> cstr!("string")\n'  
                        "3. Add 'mut' keyword for all mutable pointers in C\n"
                        "Example:\n"
                        "C: int a = 4;\n"
                        "Rust: let mut a = 4;\n"
                        "C: int array[5];\n"
                        "Rust: let mut array: Array<i32, 5> = Default::default();\n"
                        "Return only the optimized Rust code without explanations."
                    ).format(
                        sanitize_string(similar_rust_code),
                        sanitize_string(feedback),
                        sanitize_string(rust_code)
                    )
                    logging.info(f"修复专家prompt: {optimize_input}")
                    optimized = optimize_agent.generate_response(optimize_input)
                    logging.info("修复专家输出：")
                    logging.info(optimized)

                    new_optimized_code = extract_rust_code(optimized)
                    if new_optimized_code.strip():
                        with open(rust_code_file, 'r+') as f:
                            f.truncate(original_size)
                        insert_file_with_utf8(rust_code_file, new_optimized_code)
                        new_rust_code = [new_optimized_code]
                        
                        # 更新各个case函数的rust_code
                        optimized_funcs = re.finditer(r'(?:pub\s+)?fn\s+(\w+)[^{]*{[^}]*}', new_optimized_code)
                        for match in optimized_funcs:
                            func_name = match.group(1)
                            if func_name in case_funcs:
                                case_funcs[func_name]['rust_code'] = match.group(0)
                                # 更新rust签名
                                rust_signatures = re.findall(r'fn\s*([\s\S]*?){', match.group(0))
                                if rust_signatures:
                                    case_funcs[func_name]['rust_signature'] = rust_signatures[0]
                        
                        logging.info(f"代码已针对静态分析进行优化 #{static_analysis_count}\n")
                    else:
                        logging.info("警告：代码优化专家没有返回有效的Rust代码。保持原代码不变。")
                else:
                    logging.info("静态分析通过")
                    correct = True
                    break

            with open(rust_code_file, 'r+') as f:
                f.truncate(original_size)

            if not correct:
                analysis_result = static_analysis(rust_result_dir, rust_code_file)
                if not analysis_result:
                    logging.info("静态分析通过")
                    correct = True

            return correct, case_funcs[func_name]['rust_code']
        else:
            logging.info("没有成功翻译任何case函数")
            return False, ""
    else:
        # 原有的转换逻辑保持不变
        c_code = c_func_info['code']
        logging.info("开始语法转换")
        # 根据是否是测试函数选择不同的搜索函数
        if "test_" in func_name:
            similar_rust_code = find_similar_rust_test_code(c_code)
            # logging.info("使用测试代码库进行搜索")
        else:
            similar_rust_code = find_similar_rust_code(c_code)
            # logging.info("使用普通代码库进行搜索")

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

        template = Template(user_input)
        combined_syntax_input = template.substitute(c_code=c_code, similar_rust_code=similar_rust_code, rust_items=rust_items, function_call_mappings=function_call_mappings)
        # combined_syntax_input = user_input.format(c_code=c_code, similar_rust_code=similar_rust_code)
        # combined_syntax_input += """\nOnly return the function implementation without redefining any structs, variables, or types that are already defined in the codebase."""
        logging.info(f"语法专家prompt: {combined_syntax_input}")

        rust_code = syntax_agent.generate_response(combined_syntax_input, example_input, example_output)
        logging.info("语法专家输出：")
        logging.info(rust_code)
        # rust_code = similar_rust_code
        rust_code = extract_rust_code(rust_code)

        if "test_" in func_name:
            rust_code = "#[test]\n" + rust_code
            # rust_code = "#[test]\n" + similar_rust_code # 直接使用搜索的测试函数
            logging.info(f"\n添加测试标记后的代码:\n{rust_code}")
            
        # 如果是compare_string相关代码,检查并添加Ordering的导入
        if "compare_string" in rust_code_file.lower():
            # 读取现有文件内容
            existing_content = ""
            if os.path.exists(rust_code_file):
                with open(rust_code_file, 'r') as f:
                    existing_content = f.read()
            
            # 只有当文件中不存在Ordering导入时才添加
            if "use std::cmp::Ordering;" not in existing_content:
                rust_code = "use std::cmp::Ordering;\n\n" + rust_code
                logging.info(f"\n添加Ordering导入后的代码:\n{rust_code}")
            
        original_size = os.path.getsize(rust_code_file)
        original_line_num = 0
        with open(rust_code_file) as f:
            original_line_num = len(f.readlines())
        insert_file_with_utf8(rust_code_file, rust_code)
        max_static_analysis_and_test_attempts = 2
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
                logging.info("修复规划专家输出：")
                logging.info(feedback)
                # rag带上fix
                similar_rust_code = find_similar_rust_code(rust_code)
                if similar_rust_code:
                    logging.info("找到相似的Rust代码参考:")
                    logging.info(similar_rust_code)
                optimize_input = (
                    "Optimize the Rust code based on the feedback and provided reference code.\n"
                    "Reference code:\n"
                    "{}"
                    "Feedback to address:\n"
                    "{}"
                    "Code to optimize:\n"
                    "{}"
                    "\nAvailable types and utilities in translation_utils:\n"
                    "1. Pointer Types:\n"
                    "   - Vector<T>, Manual<T>, Owned<T>, Unowned<T>, SpanView<T>, Ptr<T>\n"
                    "2. Memory Operations:\n"
                    "   - c_malloc!, c_free!, c_calloc!, c_realloc!\n"
                    "   - c_memcpy!, c_memmove!, c_memset!\n"
                    "   - c_sizeof!, c_ref!\n"
                    "3. String Operations:\n"
                    "   - CStr type\n"
                    "   - c_strlen!, c_strcmp!, c_strdup!, c_sprintf!\n"
                    "   - c_tolower!, c_toupper!\n"
                    "4. Control Flow:\n"
                    "   - c_for!\n"
                    "Type Conversion Rules:\n"
                    "1. char* -> CStr\n"
                    '2. char array[] = "string" -> cstr!("string")\n'  # 使用单引号
                    "3. Add 'mut' keyword for all mutable pointers in C\n"
                    "Example:\n"
                    "C: int a = 4;\n"
                    "Rust: let mut a = 4;\n"
                    "C: int array[5];\n"
                    "Rust: let mut array: Array<i32, 5> = Default::default();\n"
                    "Return only the optimized Rust code without explanations."
                ).format(
                    sanitize_string(similar_rust_code),
                    sanitize_string(feedback),
                    sanitize_string(rust_code)
                )
                logging.info(f"修复专家prompt: {optimize_input}")
                optimized = optimize_agent.generate_response(optimize_input)
                logging.info("修复专家输出：")
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
    # 根据是否是测试文件选择不同的搜索函数
    if "test" in c_file:
        similar_rust_code = find_similar_rust_test_code(pre_code)
        # logging.info("使用测试代码库进行搜索")
    else:
        similar_rust_code = find_similar_rust_code(pre_code)
        # logging.info("使用普通代码库进行搜索")        
        
    # logging.info(f"\n找到相似的Rust代码参考:\n{similar_rust_code}")
    logging.info("开始语法转换")
    # if 'test' in c_file:
    #     combined_syntax_input = f"""\nConvert the following C code to Rust.\nC code:\n{pre_code}\nOutput only the converted Rust code without any explanations.\n"""
    # else:
    combined_syntax_input = type_convert_input_prompt_rag.format(c_code=pre_code, similar_rust_code=similar_rust_code)
    logging.info(f"语法专家prompt: {combined_syntax_input}")
    rust_code = syntax_agent.generate_response(combined_syntax_input)
    # rust_code = similar_rust_code
    logging.info("语法专家输出：")
    logging.info(rust_code)
    rust_code = extract_rust_code(rust_code)
    # rust_code = similar_rust_code
    original_size = os.path.getsize(rust_code_file)
    insert_file_with_utf8(rust_code_file, rust_code)

    max_static_analysis_and_test_attempts = 2
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
            logging.info("修复规划专家输出：")
            logging.info(feedback)
            # 结构体等Fix
            similar_rust_pre_code = find_similar_rust_code(rust_code)
            if similar_rust_pre_code:
                logging.info("找到相似的Rust代码参考:")
                logging.info(similar_rust_pre_code)
            optimize_input = (
                "Optimize the Rust code based on the feedback and provided reference code.\n"
                "Reference code:\n"
                "{}"
                "Feedback to address:\n"
                "{}"
                "Code to optimize:\n"
                "{}"
                "\nAvailable types and utilities in translation_utils:\n"
                "1. Pointer Types:\n"
                "   - Vector<T>, Manual<T>, Owned<T>, Unowned<T>, SpanView<T>, Ptr<T>\n"
                "2. Memory Operations:\n"
                "   - c_malloc!, c_free!, c_calloc!, c_realloc!\n"
                "   - c_memcpy!, c_memmove!, c_memset!\n"
                "   - c_sizeof!, c_ref!\n"
                "3. String Operations:\n"
                "   - CStr type\n"
                "   - c_strlen!, c_strcmp!, c_strdup!, c_sprintf!\n"
                "   - c_tolower!, c_toupper!\n"
                "4. Control Flow:\n"
                "   - c_for!\n"
                "Type Conversion Rules:\n"
                "1. typedef void *a -> pub type a<T: GenericValue> = T //Always use the same format generic for void*."
                "2. char* -> CStr\n"
                '3. char array[] = "string" -> cstr!("string")\n'  
                "4. Add 'mut' keyword for all mutable pointers in C\n"
                "Example:\n"
                "C: int a = 4;\n"
                "Rust: let mut a = 4;\n"
                "C: int array[5];\n"
                "Rust: let mut array: Array<i32, 5> = Default::default();\n"

                "Return only the optimized Rust code without explanations."
            ).format(
                sanitize_string(similar_rust_pre_code),
                sanitize_string(feedback),
                sanitize_string(rust_code)
            )

            logging.info(f"修复专家prompt: {optimize_input}")
            optimized = optimize_agent.generate_response(optimize_input)
            logging.info("修复专家输出：")
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
    # parser.add_argument('--file_name', default='arraylist', type=str, help='single file for test' )
    args = parser.parse_args()
    # 清空metadata
    code_preprocess("./Input/01-Primary")

    # 创建新的结果文件夹
    rust_code_dir = os.path.join(args.output_dir, "src")
    lib_file_path = os.path.join(rust_code_dir, "lib.rs")

    successful_test_count = 0
    total_test_count = 0

    # 读取元数据
    metadata = json.load(open('tool/c_metadata_all.json', 'r', encoding='utf-8'))

    # translated_flags keeps track of which functions of each file have already been translated
    translated_flags = {}
    for file_relapath, file_info in metadata.items():
        translated_flags[file_relapath] = {}
        for func_info in file_info['functions']:
            translated_flags[file_relapath][func_info['name']] = None
        
    dependencies = analyze_directory(metadata)
    translation_order = get_translation_order(dependencies)

    logging.info(f"测试文件翻译顺序：{translation_order}")
    
    # translation_order = ['test\\test-compare-functions.c', 'test\\test-hash-functions.c', 'test\\test-arraylist.c','test\\test-avl-tree.c', 'test\\test-binary-heap.c', 'test\\test-binomial-heap.c', 'test\\test-bloom-filter.c', 'test\\test-hash-table.c', 'test\\test-list.c', 'test\\test-queue.c', 'test\\test-set.c', 'test\\test-slist.c', 'test\\test-trie.c', 'test\\test-sortedarray.c', 'test\\test-rb-tree.c', 'test\\test-cpp.cpp']
    translation_order = ['test\\test-arraylist.c']
    for problem_path in translation_order:
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
        
        write_file_with_utf8(lib_file_path, f"""
#![allow(semicolon_in_expressions_from_macros)]  // 允许宏中的表达式中有分号                             
#![allow(clippy::mut_from_ref)]
pub(crate) mod translation_utils;
pub(crate) mod {test_file_name.replace('-', '_')};
""")
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
            skip_current_test = False
            for depend_func, depend_file in depend_files_and_funcs:
                if translated_flags[depend_file][depend_func] == False:
                    logging.info(f"依赖文件{depend_file}的函数{depend_func}之前翻译失败，跳过当前测试函数{test_func}的翻译")
                    skip_current_test = True
                    break

            if skip_current_test:
                continue

            # 清空依赖文件对应的rust文件
            write_file_with_utf8(rust_test_file_path, "")
            for depend_func, depend_file in depend_files_and_funcs:
                depend_file_name = os.path.splitext(os.path.basename(depend_file.replace('-', '_')))[0]
                rust_file_path = os.path.join(rust_code_dir, f"{depend_file_name}.rs") 
                write_file_with_utf8(rust_file_path, "")
                

            # 翻译依赖函数
            for depend_func, depend_file in depend_files_and_funcs:
                logging.info(f"开始翻译依赖文件{depend_file}的函数{depend_func}")
                depend_file_name = os.path.splitext(os.path.basename(depend_file.replace('-', '_')))[0]
                rust_file_path = os.path.join(rust_code_dir, f"{depend_file_name}.rs")
                
                # 构建测试环境
                content = read_file_with_auto_encoding(rust_file_path)
                if not content.strip():
                    insert_file_with_utf8(rust_file_path, """use crate::translation_utils::*;\n""")
                    for include in metadata[depend_file]['includes']:
                        match = re.search(r'#include\s*[<"]([^>"]+)[>"]', include['code'])
                        header_name = match.group(1)
                        header_base = os.path.splitext(header_name)[0]
                        if header_base in c_file_names and header_base != os.path.splitext(os.path.basename(depend_file))[0]:
                            # 检查被包含文件中的具体宏定义
                            rust_file = os.path.join(rust_code_dir, f"{header_base.replace('-', '_')}.rs")
                            logging.info(f"检查Rust文件 {rust_file} 中的宏定义")
                            
                            if os.path.exists(rust_file):
                                # 读取Rust文件内容
                                rust_content = read_file_with_auto_encoding(rust_file)
                                macros = extract_macro_definitions(rust_content)
                                
                                if macros:
                                    logging.info(f"在文件 {rust_file} 中找到宏: {macros}")
                                    # 为每个宏添加单独的use声明
                                    for macro in macros:
                                        use_line = f"use crate::{macro};\n"
                                        current_content = read_file_with_auto_encoding(rust_file_path)
                                        if use_line not in current_content:
                                            insert_file_with_utf8(rust_file_path, use_line)
                                            logging.info(f"添加宏导入: {use_line.strip()}")
                                else:
                                    logging.info(f"在文件 {rust_file} 中未找到宏定义")
                                
                                # 添加模块的常规导入
                                use_line = f"use crate::{header_base.replace('-', '_')}::*;\n"
                                current_content = read_file_with_auto_encoding(rust_file_path)
                                if use_line not in current_content:
                                    insert_file_with_utf8(rust_file_path, use_line)
                                    logging.info(f"添加模块导入: {use_line.strip()}")
                    if not metadata[depend_file]['rust_items'].strip():
                        metadata[depend_file]['rust_items'] = convert_c_initialization_to_rust(
                            depend_file,
                            rust_file_path,
                            args.output_dir,
                            metadata
                        )
                    insert_file_with_utf8(rust_file_path, metadata[depend_file]['rust_items'])

                # 如果之前翻译译过该数，则直接入翻译结果
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
                        include_file = os.path.join('src', include_file + '.c')
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
                    metadata,
                    Syntax_prompt_rag,
                    Syntax_example_input,
                    Syntax_example_output
                )
                if not success:
                    translated_flags[depend_file][depend_func] = False
                    logging.info(f"依赖文件{depend_file}的函数{depend_func}翻译失败")
                rust_signatures = re.findall(r'fn\s*([\s\S]*?){', rust_code)
                if rust_signatures:
                    c_func_info['rust_signature'] = rust_signatures[0]
                c_func_info['rust_code'] = rust_code
                insert_file_with_utf8(rust_file_path, rust_code)
            
            logging.info(f"测试文件{problem_path}的测试函数 {test_func} 的依赖项翻译完毕，开始翻译测试函数")
            
            # 构建测试文件的测试环境
            test_func_info = None
            for func_info in metadata[problem_path]['functions']:
                if func_info['name'] == test_func:
                    test_func_info = func_info
                    break

            # 读取当前文件内容，只读取一次
            current_content = read_file_with_auto_encoding(rust_test_file_path)
            imports_to_add = set()  # 使用集合来存储需要添加的导入语句

            # 添加必要的导入，包括宏
            for depend_func in test_func_info['depend_funcs']:
                if depend_func['file'] != problem_path:
                    depend_file_name = os.path.splitext(os.path.basename(depend_func['file'].replace('-', '_')))[0]
                    
                    # 检查依赖文件中是否包含宏定义
                    has_macros = False
                    if depend_func['file'] in metadata:
                        rust_code = ''
                        for func_info in metadata[depend_func['file']]['functions']:
                            if func_info.get('rust_code'):
                                rust_code += func_info['rust_code'] + '\n'
                        
                        # 检查rust_code中的导入语句
                        imports = re.findall(r'use\s+.*?;', rust_code)
                        for import_stmt in imports:
                            if import_stmt not in current_content:
                                imports_to_add.add(import_stmt)
                                
                        if 'macro_rules!' in rust_code:
                            has_macros = True
                    
                    # 构造导入语句
                    import_stmt = f"use crate::{depend_file_name}::*;"
                    if import_stmt not in current_content:
                        imports_to_add.add(import_stmt)

            # 添加 translation_utils 导入
            if "use crate::translation_utils::*;" not in current_content:
                imports_to_add.add("use crate::translation_utils::*;")

            # 一次性写入所有新的导入语句
            if imports_to_add:
                new_imports = '\n'.join(sorted(imports_to_add)) + '\n'
                insert_file_with_utf8(rust_test_file_path, new_imports)

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
                metadata,
                test_prompt_rag,
                test_example_input,
                test_example_output
            )
            rust_signatures = re.findall(r'fn\s*([\s\S]*?){', rust_test)
            if rust_signatures:
                test_func_info['rust_signature'] = rust_signatures[0]
            test_func_info['rust_code'] = rust_test
            insert_file_with_utf8(rust_test_file_path, rust_test)

            logging.info(f"测试文件{problem_path}的测试函数 {test_func} 之前的所有代码的翻译完毕，开始测试")
            max_test_count = 2
            test_count = 0

            # 将所有代码写入一个文件进行测试
            temp_test_project_dir = args.output_dir+'_temp'
            temp_test_src_dir = os.path.join(temp_test_project_dir, 'src')
            temp_test_lib_path = os.path.join(temp_test_src_dir, 'lib.rs')
            temp_test_file_path = os.path.join(temp_test_src_dir, 'temp.rs')

            write_file_with_utf8(temp_test_lib_path, """pub mod translation_utils;
pub(crate) mod temp;
""")

            depend_files = []
            for _, depend_file in depend_files_and_funcs:
                if depend_file not in depend_files:
                    depend_files.append(depend_file)

            # 将rust代码整合在一起，同时整合c代码
            rust_items_code = ''
            extra_use_codes = ''
            feedback_input_c_code = []
            success_rust_func_code = []
            feedback_input_rust_func_code = []
            under_test_count = 1
            append_flag = True
            for depend_file in depend_files:
                if depend_file == problem_path:
                    append_flag = False
                rust_items_code += metadata[depend_file]['rust_items'] + '\n\n'
                feedback_input_c_code.append('\n'.join([head['code'] for head in metadata[depend_file]['head_info']]))
                feedback_input_c_code.append('\n'.join([variable['code'] for variable in metadata[depend_file]['variables']]))
            if append_flag:
                feedback_input_c_code.append('\n'.join([head['code'] for head in metadata[problem_path]['head_info']]))
                feedback_input_c_code.append('\n'.join([variable['code'] for variable in metadata[problem_path]['variables']]))
                rust_items_code += metadata[problem_path]['rust_items'] + '\n\n'

            for depend_func, depend_file in depend_files_and_funcs:
                for func_info in metadata[depend_file]['functions']:
                    if func_info['name'] == depend_func:
                        if translated_flags[depend_file][depend_func] != True:
                            under_test_count += 1
                            feedback_input_rust_func_code.append(func_info['rust_code'])
                            feedback_input_c_code.append(func_info['code'])
                        else:
                            success_rust_func_code.append(func_info['rust_code'])
            feedback_input_c_code.append(test_func_info['code'])
            feedback_input_rust_func_code.append(test_func_info['rust_code'])

            total_rust_code = "use crate::translation_utils::*;\n" + rust_items_code + '\n' + "\n".join(success_rust_func_code) + '\n' + '\n'.join(feedback_input_rust_func_code)
            write_file_with_utf8(temp_test_file_path, total_rust_code)

            success_flag = False
            while test_count < max_test_count:
                test_count += 1
                test_success, dynamic_errors = compile_and_test_rust(test_func, temp_test_project_dir)
                if test_success and not dynamic_errors:
                    logging.info(f"测试文件{problem_path}的测试函数 {test_func} 编译和测试成功")
                    success_flag = True
                    break
                
                logging.info(f"{test_func} 测试错误\n")

                # 运行失败，进行优化
                logging.info("运行失败，继续优化")
                total_feedback_rust_code = extra_use_codes + rust_items_code+'\n'+'\n'.join(feedback_input_rust_func_code)
                feedback_input = feedback_input_prompt.format(c_code='\n'.join(feedback_input_c_code),
                                                               rust_code=total_feedback_rust_code,
                                                               error_msg=sanitize_string(dynamic_errors))
                logging.info(f"修复规划专家prompt: {feedback_input}")
                feedback = feedback_agent.generate_response(feedback_input)
                logging.info("修复规划专家输出：")
                logging.info(feedback)

                rust_functions = '\n'.join(feedback_input_rust_func_code)
                optimize_input = optimize_input_prompt.format(feedback=sanitize_string(feedback),
                                                               rust_items=rust_items_code,
                                                               functions=rust_functions)
                logging.info(f"修复专家prompt: {optimize_input}")
                optimized = optimize_agent_2.generate_response(optimize_input)
                logging.info("修复专家输出：")
                logging.info(optimized)
                new_rust_code = extract_rust_code(optimized)
                if new_rust_code.strip():
                    optimized_rust_codes, use_codes = extract_rust_funcs(new_rust_code)
                    
                    if len(optimized_rust_codes) != under_test_count:
                        logging.info("警告：代码优化专家没有返回正确的函数数量。保持原代码不变。")
                        continue
                    
                    # 更新feedback_input_rust_func_code并写入文件
                    optimized_rust_codes[-1] = '#[test]\n' + optimized_rust_codes[-1]
                    extra_use_codes = ''
                    feedback_input_rust_func_code = optimized_rust_codes.copy()
                    total_rust_code = """use crate::translation_utils::*;
""" + rust_items_code + '\n' + "\n".join(success_rust_func_code) + '\n' + '\n'.join(feedback_input_rust_func_code)
                    for use_code in use_codes:
                        if use_code not in total_rust_code:
                            extra_use_codes += use_code + '\n'
                    total_rust_code = extra_use_codes + total_rust_code
                    write_file_with_utf8(temp_test_file_path, total_rust_code)

                    logging.info(f"代码已针对运行失败进行优化 #{test_count}")
                else:
                    logging.info("警告：代码优化专家没有返回有效的Rust代码。保持原代码不变。")

            # 更新metadata
            function_count = 0
            if extra_use_codes:
                metadata[problem_path]['rust_items'] += '\n' + extra_use_codes
            for depend_func, depend_file in depend_files_and_funcs:
                if translated_flags[depend_file][depend_func] != True:
                    for func_info in metadata[depend_file]['functions']:
                        if func_info['name'] == depend_func:
                            func_info['rust_code'] = feedback_input_rust_func_code[function_count]
                            rust_signatures = re.findall(r'fn\s*([\s\S]*?){', feedback_input_rust_func_code[function_count])
                            function_count += 1
                            if rust_signatures:
                                func_info['rust_signature'] = rust_signatures[0]
                            break
            test_func_info['rust_code'] = feedback_input_rust_func_code[-1]
            rust_signatures = re.findall(r'fn\s*([\s\S]*?){', feedback_input_rust_func_code[-1])
            if rust_signatures:
                test_func_info['rust_signature'] = rust_signatures[0]
            
            # 更新translated_flags
            if success_flag:
                for depend_func, depend_file in depend_files_and_funcs:
                    translated_flags[depend_file][depend_func] = True
                translated_flags[problem_path][test_func] = True
                successful_test_count += 1
            else:
                translated_flags[problem_path][test_func_info['name']] = False
                for depend_func, depend_file in depend_files_and_funcs:
                    if translated_flags[depend_file][depend_func] == None:
                        translated_flags[depend_file][depend_func] = False
                logging.info(f"测试函数 {test_func_info['name']} 翻译失败")
    #检查哪些函数没有翻译
    # for file_name, file_info in metadata.items():
    #     for func_info in file_info['functions']:
    #         if translated_flags[file_name][func_info['name']] == None:
    #             logging.info(f"文件{file_name}的函数{func_info['name']}没有翻译")
    #             rust_items = metadata[file_name]['rust_items'] + '\n\n'
    #             rust_file_path = os.path.join(rust_code_dir, f"{os.path.splitext(os.path.basename(file_name.replace('-', '_')))[0]}.rs")
    #             success,rust_code = convert_c_funcs_to_rust(
    #                 depend_file,
    #                 depend_func,
    #                 c_func_info,
    #                 rust_items,
    #                 rust_file_path,
    #                 args.output_dir,
    #                 metadata,
    #                 Syntax_prompt_rag,
    #                 Syntax_example_input,
    #                 Syntax_example_output
    #             )
    #             rust_signatures = re.findall(r'fn\s*([\s\S]*?){', rust_code)
    #             if rust_signatures:
    #                 func_info['rust_signature'] = rust_signatures[0]
    #             func_info['rust_code'] = rust_code
    # Write back metadata to file
    with open('tool/c_metadata_all.json', 'w', encoding='utf-8') as f:
        json.dump(metadata, f, indent=2, ensure_ascii=False)

    logging.info(f"测试函数总数：{total_test_count}")
    logging.info(f"测试成功数：{successful_test_count}")
    logging.info(f"测试成功率：{(successful_test_count/total_test_count * 100):.2f}%")
def create_temp_rust_project(project_path: str, project_name) -> bool:
    """
    在指定目录下创建一个新的Rust库项目，如果项目已存在则跳过
    """
    try:
        if os.path.exists(project_path):
            if os.path.exists(os.path.join(project_path, "Cargo.toml")):
                logging.info(f"Rust project already exists at {project_path}")
                return True
            else:
                logging.error(f"Directory exists but is not a Rust project: {project_path}")
                return False
        
        os.makedirs(os.path.dirname(project_path), exist_ok=True)
        
        result = subprocess.run(
            ['cargo', 'new', '--lib', project_path],
            capture_output=True,
            text=True,
            check=True
        )

        cargo_content = f"""[package]
name = "{project_name}"
version = "0.1.0"
edition = "2021"

[lib]
name         = "{project_name}"
crate-type   = ["rlib", "cdylib"]

[profile.dev]
overflow-checks = false
opt-level = 3

[dependencies]
lazy_static = "1.5.0"
libc = "0.2.155"
"""
        write_file_with_utf8(os.path.join(project_path, 'Cargo.toml'), cargo_content)

        # Copy translation_utils directory to project src directory
        translation_utils_src = 'tool/translation_utils/'
        translation_utils_dest = os.path.join(project_path, 'src/translation_utils/')
        if os.path.exists(translation_utils_src):
            os.makedirs(translation_utils_dest, exist_ok=True)
            for root, dirs, files in os.walk(translation_utils_src):
                rel_path = os.path.relpath(root, translation_utils_src)
                dest_dir = os.path.join(translation_utils_dest, rel_path)
                os.makedirs(dest_dir, exist_ok=True)
                for file_name in files:
                    src_file = os.path.join(root, file_name)
                    dest_file = os.path.join(dest_dir, file_name)
                    shutil.copy2(src_file, dest_file)
        
        if result.returncode == 0:
            logging.info(f"Successfully created Rust project at {project_path}")
            return True
        else:
            logging.error(f"Failed to create Rust project: {result.stderr}")
            return False
            
    except Exception as e:
        logging.error(f"Error creating Rust project: {str(e)}")
        return False

def setup_projects():
    """创建主项目和临时项目"""
    base_dir = os.path.join(os.getcwd(), "Output")
    primary_path = os.path.join(base_dir, "primary")
    temp_path = os.path.join(base_dir, "primary_temp")
    
    if not create_temp_rust_project(primary_path, "primary"):
        raise Exception("Failed to create primary project")
    if not create_temp_rust_project(temp_path, "primary_temp"):
        raise Exception("Failed to create temporary project")

def cleanup_projects():
    """
    删除主项目和临时项目目录
    Returns:
        bool: 删除是否成功
    """
    try:
        base_dir = os.path.join(os.getcwd(), "Output")
        primary_path = os.path.join(base_dir, "primary")
        temp_path = os.path.join(base_dir, "primary_temp")
        
        # 删除主项目
        if os.path.exists(primary_path):
            shutil.rmtree(primary_path)
            logging.info(f"Successfully removed primary project at {primary_path}")
            
        # 删除临时项目    
        if os.path.exists(temp_path):
            shutil.rmtree(temp_path)
            logging.info(f"Successfully removed temporary project at {temp_path}")
            
        return True
            
    except Exception as e:
        logging.error(f"Error cleaning up projects: {str(e)}")
        return False

if __name__ == "__main__":
    setup_logging()
    setup_projects()
    process_files()
    cleanup_projects()