from Agent import *
from prompts import *
from split_function import *
from extract_component import *
from add_crate import *
from add_stest_functions import * 
from remove_stest_functions import *
from remove_alloc_test_blocks import *
import subprocess
import re
import logging
import shutil
import time
import select


agent = Agent(
    role="代码优化",
    prompt="You are a proficient C and Rust advanced developer.",
    temperature=0.3,
    top_p=0.8
)
PROJ_PATH="../output/primary"
C2RUST_PATH=f"{PROJ_PATH}-c2rust"
SRC_PATH=os.path.join(PROJ_PATH, "src")
TEST_PATH=os.path.join(PROJ_PATH, "tests")
C_PATH="../input/01-Primary"
COMPARE_HASH_SRC_LIST = [
    f"{SRC_PATH}/compare_int.rs",
    f"{SRC_PATH}/compare_string.rs",
    f"{SRC_PATH}/compare_pointer.rs",
    f"{SRC_PATH}/hash_int.rs",
    f"{SRC_PATH}/hash_string.rs",
    f"{SRC_PATH}/hash_pointer.rs"
]
MAX_ATTEMPS=7
TEST_TIMEOUT=10
FIRST_TEST_TIMEOUT=300

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


def check_cur_path():
    current_directory = os.getcwd()
    if os.path.basename(current_directory)=="tool":
        return True
    return False

def cargo_test(timeout=TEST_TIMEOUT):
    current_directory = os.getcwd()
    os.chdir(f"{PROJ_PATH}")
    # 运行 cargo test 命令
    env = os.environ.copy()
    env["RUSTFLAGS"] = "-Awarnings"
    command = ["cargo", "test", "--", "--test-threads=1"]
    # 第一次构建，可能有crates更新   
    try:
        result = subprocess.run(
            command,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            text=True,
            env=env,
            timeout=timeout
        )
        os.chdir(current_directory)
        if result.returncode == 0:
            logging.info("所有测试通过！")
            return (True, "")
        else:
            logging.info("测试未通过！")
            error_msg = result.stderr
            logging.info(error_msg)
            return (False, error_msg)
    except subprocess.TimeoutExpired:
        logging.info("超时")
        if timeout == FIRST_TEST_TIMEOUT:
            logging.info("可能是cargo crates未能更新，请在项目路径下手动运行cargo test -- --test-threads=1检查")
        os.chdir(current_directory)
        return (False, "Timeout error, Please ensure that your program does not cause errors or infinite loops.")

# 从```中提取rust代码
# 返回代码/""
def extract_rust_code(review_text: str) -> str:
    if review_text is None or review_text == "":
        return ""
    code_blocks = re.findall(r'```rust\n(.*?)```', review_text, re.DOTALL)
    if code_blocks:
        review_text = code_blocks[0].strip()
    review_text = re.sub(r'//.*$', '', review_text, flags=re.MULTILINE)
    return review_text.strip()

# 将文件中的旧代码替换为新代码
# 如果找不到目标段落，返回F
def function_replace(file_path, origin_code, new_code):
    with open(file_path,'r') as file:
        content = file.read()
    if origin_code not in content:
        logging.info(f"下列原函数未能在文件{file_path}中查找到：\n{origin_code}")
        return False
    with open(file_path,'w') as file:
        file.write(content.replace(origin_code, new_code))
    return True

# 将文件中的新代码还原为旧代码
# 如果找不到目标段落，返回F
def function_restore(file_path, origin_code, new_code):
    with open(file_path,'r') as file:
        content = file.read()
    if new_code not in content:
        logging.info(f"下列新函数未能在文件{file_path}中查找到：\n{new_code}")
        return False
    with open(file_path,'w') as file:
        file.write(content.replace(new_code, origin_code))
    return True

# 将文件中的指定代码注释掉（每行前加//)
# 返回F/T
def comment_code(file_path, target_code):
    with open(file_path,'r') as file:
        content = file.read()
    if target_code not in content:
        logging.info(f"未能在文件{file_path}中找到目标代码：\n{target_code}")
        return False
    commented_code = '\n'.join(f'// {line}' for line in target_code.split('\n'))
    with open(file_path,'w') as file:
        # file.write(content.replace(target_code, commented_code))
        file.write(content.replace(f"{target_code}\n", ''))
    return True


# 对指定的代码（origin_code）迭代优化
# 需传入prompt（包括输入输出示例和user_ask）
# 返回T/F
def optimize(file_path, origin_code, user_ask, example_input=[], example_output=[], opt_round=-1):
    cnt = 0
    flag = False
    cur_user_ask = user_ask
    while cnt < MAX_ATTEMPS:
        logging.info("prompt输入：")
        logging.info(cur_user_ask)
        output = agent.generate_response(user_input=cur_user_ask, example_input=example_input, example_output=example_output)
        output = extract_rust_code(output)
        logging.info("LLM输出：")
        logging.info(output)
        if (output == ""):
            logging.info("返回结果为空，下一轮优化")
            continue
        logging.info("替换并测试")
        function_replace(file_path, origin_code=origin_code, new_code=output)
        res, error_msg = cargo_test()
        if (res):
            flag = True
            break
        logging.info(error_msg)
        logging.info(f"cnt={cnt} 测试失败，还原重试")
        function_restore(file_path, origin_code=origin_code, new_code=output)
        cnt += 1
        # 补充错误信息
        if opt_round == 2 and "process didn't exit successfully" in error_msg:
            cur_user_ask = user_ask + fix_prompt_memory.format(last_code=output)
        else:
            cur_user_ask = user_ask + fix_prompt.format(last_code=output, error_msg=error_msg)
    if flag:
        return True
    else:
        return False

# 遍历文件中的函数，进行assert宏优化
def assert_optimize(file_path):
    functions = split_rust_functions_in_file(file_path)
    with open(file_path,'r') as file:
        origin_content = file.read()
    for func in functions:
        if "s_test" in func['name'] or "out_of_memory" in func['name']:
            continue
        origin_code = func['code']
        if origin_code not in origin_content:
            logging.info(f"下列原函数未能在文件{file_path}中查找到：\n{origin_code}")
            continue
        user_ask = assert_optimize_prompt.format(rust_code=origin_code)
        res = optimize(file_path, origin_code=origin_code, user_ask=user_ask, opt_round=0)
        if not res:
            logging.info(f"函数{func['name']}assert优化失败")

# 遍历文件中的函数，进行第一轮安全优化    
def first_safe_optimize(file_path):
    functions = split_rust_functions_in_file(file_path)
    fail_list = []
    with open(file_path,'r') as file:
        origin_content = file.read()
    example_input = [
        first_optimize_prompt.format(rust_code=src_codein_1),
        
    ]
    example_output = [
        src_codeout_1,
    ]
    for func in functions:
        if "s_test" in func['name'] or "out_of_memory" in func['name']:
            continue
        origin_code = func['code']
        if origin_code not in origin_content:
            logging.info(f"下列原函数未能在文件{file_path}中查找到：\n{origin_code}")
            continue

        user_ask = first_optimize_prompt.format(rust_code=origin_code)
        res = optimize(file_path, origin_code=origin_code, user_ask=user_ask, example_input=example_input, example_output=example_output, opt_round=1)
        if not res:
            logging.info(f"函数{func['name']}第一轮优化失败")
            fail_list.append(func['name'])
    return fail_list

def second_safe_optimize(file_path):
    functions = split_rust_functions_in_file(file_path)
    with open(file_path,'r') as file:
        origin_content = file.read()

    # 获取静态变量
    static_code = '\n'.join(extract_static_declaration(path=file_path))
    # 获取extern C部分，利用extract_rust_code去注释
    extern_code = extract_rust_code(extract_extern_declaration(path=file_path))

    if "test_" in file_path:
        logging.info("test file")
        example_in_1 = second_optimize_prompt.format(rust_code=test_second_opt_example_code_1, 
                                                static_variables=test_second_opt_example_static_1, 
                                                extern_C=test_second_opt_example_extern_C_1)
        example_in_2 = second_optimize_prompt.format(rust_code=test_second_opt_example_code_2, 
                                                static_variables=test_second_opt_example_static_2, 
                                                extern_C=test_second_opt_example_extern_C_2)
        example_input = [
            example_in_1, example_in_2
        ]
        example_output = [
            test_second_opt_example_output_1,test_second_opt_example_output_2
        ]
    else:
        example_in_1 = second_optimize_prompt.format(rust_code=src_second_opt_example_code_1, 
                                                static_variables=src_second_opt_example_static_1, 
                                                extern_C=src_second_opt_example_extern_C_1)
        example_in_2 = second_optimize_prompt.format(rust_code=src_second_opt_example_code_2, 
                                                static_variables=src_second_opt_example_static_2, 
                                                extern_C=src_second_opt_example_extern_C_2)
        example_in_3 = second_optimize_prompt.format(rust_code=src_second_opt_example_code_3, 
                                                static_variables=src_second_opt_example_static_3, 
                                                extern_C=src_second_opt_example_extern_C_3)
        example_input = [
            example_in_1, example_in_2, example_in_3
        ]
        example_output = [
            src_second_opt_example_output_1, src_second_opt_example_output_2, src_second_opt_example_output_3
        ]
    # for e in example_input:
    #     print(e)
    # for e in example_output:
    #     print(e)
    # print(example_input)
    # print(example_output)
    for func in functions:
        if "s_test" in func['name'] or "out_of_memory" in func['name']:
            continue
        # if "test_list_remove_data" not in func['name']:
        #     continue
        origin_code = func['code']
        if origin_code not in origin_content:
            logging.info(f"下列原函数未能在文件{file_path}中查找到：\n{origin_code}")
            continue
        user_ask = second_optimize_prompt.format(rust_code=origin_code, static_variables=static_code, extern_C=extern_code)
        res = optimize(file_path, origin_code=origin_code, user_ask=user_ask, example_input=example_input, example_output=example_output, opt_round=2)
        if not res:
            logging.info(f"函数{func['name']}第二轮优化失败")
       


# 移除（注释）test 文件中的extern C函数声明
# fail_list中为未能转为safe的函数名，保留其extern C声明
def remove_extern_declaration_for_testfile(file_path, fail_list):
    # 文件名 test_avl_tree.rs
    file_name = os.path.basename(file_path)
    # 提取模块名 avl_tree
    start_index = len("test_")
    end_index = file_name.find(".rs")
    module_name = file_name[start_index:end_index]

    remove_list = set()
    for file in COMPARE_HASH_SRC_LIST:
        remove_list = remove_list | set(extract_function_names(file))
    if "hash_functions" not in module_name and "compare_functions" not in module_name:
        remove_list = remove_list | set(extract_function_names(f"{SRC_PATH}/{module_name}.rs"))
    remove_list = remove_list - set(fail_list)

    extern_decl = extract_extern_declaration(file_path)
    func_name_to_sig = extract_func_signature_from_extern_declaration(extern_decl)
    for func_name in func_name_to_sig.keys():
        if func_name in remove_list or "run_tests" in func_name or "alloc_test" in func_name:
            comment_code(file_path=file_path, target_code=func_name_to_sig[func_name])

def remove_extern_declaration_for_srcfile(file_path):
    extern_decl = extract_extern_declaration(file_path)
    func_name_to_sig = extract_func_signature_from_extern_declaration(extern_decl)
    for func_name in func_name_to_sig.keys():
        if "alloc_test" in func_name:
            comment_code(file_path=file_path, target_code=func_name_to_sig[func_name])


# 移除（注释）test 文件中的结构体声明
def remove_struct_and_type_declaration(file_path):
    struct_codes = extract_struct_declaration(file_path)
    logging.info(f"查找到{len(struct_codes)}个结构体定义")
    for struct in struct_codes:
        comment_code(file_path, struct)
    attribute="#[derive(Copy, Clone)]\n#[repr(C)]"
    comment_code(file_path, attribute)

    type_codes = extract_type_declaration(file_path)
    logging.info(f"查找到{len(type_codes)}个type定义")
    for type in type_codes:
        comment_code(file_path, type)

# 移除测试文件中的main函数，以及与run_test相关的东西
def remove_main_and_tests(file_path):
    functions = split_rust_functions_in_file(file_path)
    for func in functions:
        if "main" in func['name']:
            func_code = func['code']
            comment_code(file_path, func_code)

    static_codes = extract_static_declaration(file_path)
    for static in static_codes:
        # print(static)
        if "UnitTestFunction" in static:
            comment_code(file_path, static)

# 从c2rust的翻译结果构造rust项目
def construct_project():
    if os.path.exists(f"{C2RUST_PATH}/src/src") and os.path.exists(f"{C2RUST_PATH}/src/test"):
        if not os.path.exists(f"{PROJ_PATH}"):
            logging.info("项目文件夹不存在，新建cargo项目")
            result = subprocess.run(
                ["cargo", "new", f"{PROJ_PATH}"],
                stdout=subprocess.PIPE,
                stderr=subprocess.PIPE,
                text=True,
            )
            with open(f"{PROJ_PATH}/Cargo.toml", 'a') as file:
                file.write('libc = "0.2.167"\n')
            shutil.rmtree(f"{PROJ_PATH}/src")
        else:
            logging.info("项目文件夹存在，移除src、tests目录")
            if os.path.exists(f"{PROJ_PATH}/src"):
                shutil.rmtree(f"{PROJ_PATH}/src")
            if os.path.exists(f"{PROJ_PATH}/tests"):
                shutil.rmtree(f"{PROJ_PATH}/tests")

        shutil.copytree(f"{C2RUST_PATH}/src/src", f"{PROJ_PATH}/src")
        shutil.copytree(f"{C2RUST_PATH}/src/test", f"{PROJ_PATH}/tests")
    os.remove(f"{PROJ_PATH}/tests/framework.rs")
    shutil.move(f"{PROJ_PATH}/tests/alloc_testing.rs", f"{PROJ_PATH}/src/alloc_testing.rs")
    if not os.path.exists(f"{PROJ_PATH}/count_safe_ratio.sh"):
        shutil.copy("./count_safe_ratio.sh", f"{PROJ_PATH}")
        subprocess.run(['chmod', '+x', f"{PROJ_PATH}/count_safe_ratio.sh"])

    add_crate_and_lib(PROJ_PATH)
    add_stest_functions(PROJ_PATH)
    


src_fail_dict = {}
test_fail_dict = {}
if __name__ == "__main__":
    # 全是相对路径，确认一下工作目录
    if not check_cur_path():
        logging.info("请在tool目录下运行")
        exit(1)
    setup_logging()
    if os.path.exists(C2RUST_PATH):
        logging.info("c2rust文件夹存在，清空")
        shutil.rmtree(C2RUST_PATH)
    os.makedirs(C2RUST_PATH)


    # C代码预处理
    logging.info("预处理C代码")
    remove_alloc_test_blocks_in_dir(f"{C_PATH}/test")
    # C2Rust
    logging.info("c2rust转换")
    c2rust_start_time = time.time()
    result = subprocess.run(
        ["../c2rust/target/debug/c2rust", "transpile", "--output-dir", f"{C2RUST_PATH}", f"{C_PATH}/compile_commands.json"],
        text=True,
    )
    if result.returncode == 0:
        logging.info("c2rust 转换成功")
    else:
        logging.info("c2rust 转换失败")
        exit(2)
    # 构建项目并测试
    logging.info("项目构建")
    construct_project()
    logging.info("c2rust 测试")
    cargo_test(FIRST_TEST_TIMEOUT)

    c2rust_end_time = time.time()
    logging.info(f"c2rust转换耗时{c2rust_end_time - c2rust_start_time}秒")

    # 调试用
    DEBUG_FLAG=True
    DEBUG_TARGET="queue"
    # assert宏替换
    assert_start_time = time.time()
    for root, dirs, files in os.walk(TEST_PATH):
        for file in files:
            if DEBUG_FLAG and DEBUG_TARGET not in file:
                continue
            logging.info(f"注释{file}中的main函数")
            remove_main_and_tests(os.path.join(root, file))
            logging.info("assert宏替换")
            assert_optimize(os.path.join(root, file))
    for root, dirs, files in os.walk(SRC_PATH):
        for file in files:
            if  DEBUG_FLAG and DEBUG_TARGET not in file:
                continue
            if "alloc_testing" in file:
                logging.info("当前文件为alloc_testing，先进行assert替换")
                assert_optimize(file_path = os.path.join(root, file))

    assert_end_time = time.time()
    logging.info(f"assert宏替换完成，耗时{assert_end_time - assert_start_time}秒")


    # 第一轮优化
    first_start_time = time.time()
    for root, dirs, files in os.walk(SRC_PATH):
        for file in files:
            if  DEBUG_FLAG and DEBUG_TARGET not in file and "alloc_testing" not in file:
                continue
            module_name = file.split('.')[0]
            file_path = os.path.join(root, file)

            logging.info(f"{file}第一轮优化")
            fail_list = first_safe_optimize(file_path)
            logging.info(f"第一轮优化失败函数：{fail_list}")
            src_fail_dict[module_name] = fail_list

    for root, dirs, files in os.walk(TEST_PATH):
        for file in files:
            if  DEBUG_FLAG and DEBUG_TARGET not in file and "alloc_testing" not in file:
                continue
            start_index = len("test_")
            end_index = file.find(".rs")
            module_name = file[start_index:end_index]
            file_path = os.path.join(root, file)

            logging.info(f"{file}第一轮优化")
            

            fail_list = first_safe_optimize(file_path)
            logging.info(f"第一轮优化失败函数：{fail_list}")
            test_fail_dict[module_name] = fail_list
    first_end_time = time.time()
    logging.info(f"第一轮优化完成，耗时{first_end_time - first_start_time}秒")

    logging.info("统计safe指标")
    remove_stest_functions(PROJ_PATH)
    # 执行脚本并捕获输出
    current_directory = os.getcwd()
    os.chdir(f"{PROJ_PATH}")
    process = subprocess.Popen([f"./count_safe_ratio.sh"], stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)
    # 获取输出和错误
    stdout, _ = process.communicate()
    logging.info(f"{stdout}")
    os.chdir(current_directory)
    add_stest_functions(PROJ_PATH)
    
    ignore = ["binomial_heap.rs","compare_string.rs","hash_table.rs","avl_tree.rs","hash_pointer.rs","compare_pointer.rs","sortedarray.rs",
              "set.rs", "list.rs", "rb_tree.rs", "hash_string.rs", "alloc_testing.rs", "slist.rs"
              ]
    # 第二轮优化
    second_start_time = time.time()
    for root, dirs, files in os.walk(SRC_PATH):
        for file in files:
            if  DEBUG_FLAG and DEBUG_TARGET not in file:
                continue
            if file in ignore:
                continue
            logging.info(f"{file}第二轮优化")

            module_name = file.split('.')[0]
            file_path = os.path.join(root, file)

            remove_extern_declaration_for_srcfile(file_path)
            second_safe_optimize(file_path)


    for root, dirs, files in os.walk(TEST_PATH):
        for file in files:
            if  DEBUG_FLAG and DEBUG_TARGET not in file:
                continue
            start_index = len("test_")
            end_index = file.find(".rs")
            module_name = file[start_index:end_index]
            file_path = os.path.join(root, file)

            fail_list = []
            # 获取src中转化失败的函数
            # 移除（注释）test 文件中的extern C函数声明
            if src_fail_dict.get(module_name):
                fail_list += src_fail_dict[module_name]
            if test_fail_dict.get(module_name):
                fail_list += test_fail_dict[module_name]
            logging.info(f"注释{file}中的extern C函数声明，fail list={fail_list}")
            remove_extern_declaration_for_testfile(file_path, fail_list)
            # 移除（注释）test 文件中的结构体声明（和src的重复定义了）
            logging.info(f"注释{file}中的全局结构体和类型声明")
            remove_struct_and_type_declaration(file_path)
            
            logging.info(f"{file}第二轮优化")
            second_safe_optimize(file_path)
    second_end_time = time.time()
    logging.info(f"第二轮优化完成，耗时{second_end_time - second_start_time}秒")

    logging.info("统计safe指标")
    remove_stest_functions(PROJ_PATH)
    # 执行脚本并捕获输出
    current_directory = os.getcwd()
    os.chdir(f"{PROJ_PATH}")
    process = subprocess.Popen([f"./count_safe_ratio.sh"], stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)
    # 获取输出和错误
    stdout, _ = process.communicate()
    logging.info(f"{stdout}")
    os.chdir(current_directory)


    
