# import os
# import re
# import shutil
# import tempfile
# from Agents import *
# import subprocess
# import sys
# import chardet
# import unicodedata
# # 设置默认编码为UTF-8
# sys.stdout.reconfigure(encoding='utf-8')
# sys.stderr.reconfigure(encoding='utf-8')


# from openai import OpenAI

# siliconflow_key = "sk-rawkqgnttfoqtgivkdtisvzsaymhmhtrshrfdaolsiahfbjt"
# siliconflow_base_url = "https://api.siliconflow.cn/v1"

# deepseek_key = "sk-764372d3e899489480a9a0deda637953"
# deepseek_base_url = "https://api.deepseek.com/beta"
# client = OpenAI(api_key=siliconflow_key, base_url=siliconflow_base_url)

# class Agent:
#     def __init__(self, role: str, prompt: str, temperature: float, top_p: float):
#         self.role = role
#         self.prompt = prompt
#         self.temperature = temperature
#         self.top_p = top_p

#     def generate_response(self, user_input: str) -> str:
#         print(f"{self.role} 正在回答:")
#         try:
#             response = client.chat.completions.create(
#                 model="deepseek-ai/DeepSeek-V2.5",

#                 messages=[
#                     {"role": "user", "content": user_input}
#                 ],
#                 temperature=self.temperature,
#                 top_p= self.top_p,
#                 stream=True
#             )
#             full_response = ""
#             for chunk in response:
#                 if chunk.choices and chunk.choices[0].delta.content is not None:
#                     content = chunk.choices[0].delta.content
#                     print(content, end='', flush=True)
#                     full_response += content
#             print("\n")
#             return full_response
#         except Exception as e:
#             print(f"生成响应时发生错误：{str(e)}")
#             return ""
        
# fix_agent = Agent(
#     role="fix Expert",
#     prompt="",
#     temperature=0.3,
#     top_p=0.85)

# DEBUG = True
# def debug_print(message):
#     if DEBUG:
#         print(message)


# def extract_rust_code(review_text: str) -> str:
#     if review_text is None:
#         print("警告：从该专家收到空响应")
#         return ""
#     code_blocks = re.findall(r'```rust\n(.*?)```', review_text, re.DOTALL)
#     if code_blocks:
#         return code_blocks[0].strip()
#     print("警告：无法从该专家的回复中提取 Rust 代码")
#     return ""

# def read_file_with_auto_encoding(file_path):
#     encoding = detect_encoding(file_path)
#     with open(file_path, 'r', encoding=encoding, errors='replace') as file:
#         return file.read()

# def write_file_with_utf8(file_path, content):
#     with open(file_path, 'w', encoding='utf-8', errors='ignore') as file:
#         file.write(content)

# c_code = """
# #define ALLOC_TEST_MAGIC 0x72ec82d2
# #define MALLOC_PATTERN 0xBAADF00D
# #define FREE_PATTERN 0xDEADBEEF
# typedef struct _BlockHeader BlockHeader;
# struct _BlockHeader {
# 	unsigned int magic_number;
# 	size_t bytes;
# };
# static size_t allocated_bytes = 0;
# signed int allocation_limit = -1;
# static BlockHeader *alloc_test_get_header(void *ptr)
# {
# 	BlockHeader *result;
# 	result = ((BlockHeader *) ptr) - 1;
# 	assert(result->magic_number == ALLOC_TEST_MAGIC);
# 	return result;
# }
# static void alloc_test_overwrite(void *ptr, size_t length,
#                                  unsigned int pattern)
# {
# 	unsigned char *byte_ptr;
# 	int pattern_seq;
# 	unsigned char b;
# 	size_t i;
# 	byte_ptr = ptr;
# 	for (i=0; i<length; ++i) {
# 		pattern_seq = (int) (i & 3);
# 		b = (unsigned char) ((pattern >> (8 * pattern_seq)) & 0xff);
# 		byte_ptr[i] = b;
# 	}
# }
# """

# # c_code="""
# # #include <limits.h>
# # #include <math.h>
# # #include <stdbool.h>
# # #include <stdio.h>
# # #include <stdlib.h>
 
# # typedef long long llong_t;
# # struct PrimeArray {
# #     llong_t *ptr;
# #     size_t size;
# #     size_t capacity;
# # };
 
# # struct PrimeArray allocate() {
# #     struct PrimeArray primes;
 
# #     primes.size = 0;
# #     primes.capacity = 10;
# #     primes.ptr = malloc(primes.capacity * sizeof(llong_t));
 
# #     return primes;
# # }
 
# # void deallocate(struct PrimeArray *primes) {
# #     free(primes->ptr);
# #     primes->ptr = NULL;
# # }
 
# # void push_back(struct PrimeArray *primes, llong_t p) {
# #     if (primes->size >= primes->capacity) {
# #         size_t new_capacity = (3 * primes->capacity) / 2 + 1;
# #         llong_t *temp = realloc(primes->ptr, new_capacity * sizeof(llong_t));
# #         if (NULL == temp) {
# #             fprintf(stderr, "Failed to reallocate the prime array.");
# #             exit(1);
# #         } else {
# #             primes->ptr = temp;
# #             primes->capacity = new_capacity;
# #         }
# #     }
 
# #     primes->ptr[primes->size++] = p;
# # }
 
# # int main() {
# #     const int cutOff = 200, bigUn = 10000, chunks = 50, little = bigUn / chunks;
# #     struct PrimeArray primes = allocate();
# #     int c = 0;
# #     bool showEach = true;
# #     llong_t u = 0, v = 1, i;
 
# #     push_back(&primes, 3);
# #     push_back(&primes, 5);
 
# #     printf("The first %d cuban primes:\n", cutOff);
# #     for (i = 1; i < LLONG_MAX; ++i) {
# #         bool found = false;
# #         llong_t mx = ceil(sqrt(v += (u += 6)));
# #         llong_t j;
 
# #         for (j = 0; j < primes.size; ++j) {
# #             if (primes.ptr[j] > mx) {
# #                 break;
# #             }
# #             if (v % primes.ptr[j] == 0) {
# #                 found = true;
# #                 break;
# #             }
# #         }
# #         if (!found) {
# #             c += 1;
# #             if (showEach) {
# #                 llong_t z;
# #                 for (z = primes.ptr[primes.size - 1] + 2; z <= v - 2; z += 2) {
# #                     bool fnd = false;
 
# #                     for (j = 0; j < primes.size; ++j) {
# #                         if (primes.ptr[j] > mx) {
# #                             break;
# #                         }
# #                         if (z % primes.ptr[j] == 0) {
# #                             fnd = true;
# #                             break;
# #                         }
# #                     }
# #                     if (!fnd) {
# #                         push_back(&primes, z);
# #                     }
# #                 }
# #                 push_back(&primes, v);
# #                 printf("%11lld", v);
# #                 if (c % 10 == 0) {
# #                     printf("\n");
# #                 }
# #                 if (c == cutOff) {
# #                     showEach = false;
# #                     printf("\nProgress to the %dth cuban prime: ", bigUn);
# #                 }
# #             }
# #             if (c % little == 0) {
# #                 printf(".");
# #                 if (c == bigUn) {
# #                     break;
# #                 }
# #             }
# #         }
# #     }
# #     printf("\nThe %dth cuban prime is %lld\n", c, v);
 
# #     deallocate(&primes);
# #     return 0;
# # }
# # """

# api_prompt="""
#         Extract and convert only the C-specific APIs to their Rust equivalents:
#         {c_code}
# """

# convert_prompt = """
#     Convert the following C code to Rust using the provided API mappings:
#     C code:
#     {c_code}
#     API mappings:
#     {api_conversion}
#     Remember to output only the converted Rust code without any explanations.
# """
# # fix_prompt="""
# #     Given the following error on the following code snippets, output the changelog. 
# #     {error_block}
# #     ---
# #     {code_snippets}
# # """
# fix_prompt="""
# You are given the below error from clippy and Rust code snippets from one or more '. rs ' files related to this error.
# error:
# {error_block}
# ---
# code snippets:
# {code_snippets}
# Instructions: 
# Fix the error on the above code snippets. 
# Not every snippet might require a fix or be relevant to the error, but take into account the code in all above snippets as it could help you derive the best possible fix.
# Assume that the snippets might not be complete and could be missing lines above or below. 
# Do not add comments or code that is not necessary to fix the error. 
# Do not use unsafe or unstable features ( through '#![ feature (...) ]').
# For your answer, return one or more ChangeLog groups, each containing one or more fixes to the above code snippets. 
# Each group must be formatted with the below instructions.
# Format instructions: 
# Each ChangeLog group must start with a description of its included fixes. 
# The group must then list one or more pairs of ( OriginalCode , FixedCode ) code snippets. 
# Each OriginalCode snippet must list all consecutive original lines of code that must be replaced ( including a few lines before and after the fixes ), 
# followed by the FixedCode snippet with all consecutive fixed lines of code that must replace the original lines of code ( including the same few lines before and after the changes ).
# In each pair, the OriginalCode and FixedCode snippets must start at the same source code line number N. 
# Each listed code line, in both the OriginalCode and FixedCode snippets, must be prefixed with [ N ] that matches the line index N in the above snippets, and then be prefixed with exactly the same whitespace indentation as the original snippets above.
# ---
# ChangeLog :1 @ <file>
# FixDescription : <summary>.
# OriginalCode@4 -6:
# [4] <white space> <original code line>
# [5] <white space> <original code line>
# [6] <white space> <original code line>
# FixedCode@4 -6:
# [4] <white space> <fixed code line>
# [5] <white space> <fixed code line>
# [6] <white space> <fixed code line>
# OriginalCode@9 -10:
# [9] <white space> <original code line>
# [10] <white space> <original code line>
# FixedCode@9 -9:
# [9] <white space> <fixed code line>
# ...
# ChangeLog : K@ <file>
# FixDescription : <summary>.
# OriginalCode@15 -16:
# [15] <white space> <original code line>
# [16] <white space> <original code line>
# FixedCode@15 -17:
# [15] <white space> <fixed code line>
# [16] <white space> <fixed code line>
# [17] <white space> <fixed code line>
# OriginalCode@23 -23:
# [23] <white space> <original code line>
# FixedCode@23 -23:
# [23] <white space> <fixed code line>
# ---
# Answer:
# """

# def static_analysis(rust_code: str) -> str:
#     with tempfile.TemporaryDirectory() as tmpdir:
#         cur_dir = os.getcwd()
#         rust_pdb = os.path.join(cur_dir, "temp.pdb")
#         rust_exe = os.path.join(cur_dir, "temp.exe")
#         rust_file = os.path.join(tmpdir, "temp.rs")
#         write_file_with_utf8(rust_file, rust_code)

#         # 使用 clippy 进行静态分析
#         clippy_result = subprocess.run(["clippy-driver", rust_file],
#                                        capture_output=True, text=True, encoding="utf-8")

        

#         # if os.path.exists("tmp_proj"):
#         #     shutil.rmtree("tmp_proj")
#         # subprocess.run(["cargo", "new", "tmp_proj"], check=True)
#         # os.chdir(os.path.join(cur_dir, "tmp_proj"))
#         # rust_file = os.path.join(cur_dir, "tmp_proj/temp.rs")
#         # write_file_with_utf8(rust_file, rust_code)
#         # clippy_result = subprocess.run(["cargo", "clippy"],
#         #                                capture_output=True, text=True, encoding="utf-8")
#         # os.chdir(cur_dir)

        
        
#         if os.path.exists(rust_pdb):
#             os.remove(rust_pdb)
#         if os.path.exists(rust_exe):
#             os.remove(rust_exe)

#         issues = []

#         # 解析 clippy 输出，只关注错误
#         clippy_output = clippy_result.stdout + clippy_result.stderr
#         print("clippy_output:")
#         print(clippy_output)

#         error_count = 0
#         error_blocks = re.findall(r'error.*?\n\n', clippy_output, re.DOTALL)
#         for block in error_blocks:
#             if block.startswith("error["):
#                 # 过滤缺少主函数错误
#                 if "E0601" in block:
#                     continue
#                 issues.append(block.strip())
#                 error_count = error_count + 1
            

#         # 提取错误代码并获取详细说明
#         # error_codes = set(re.findall(r'error\[E(\d+)]', clippy_output))
#         # for code in error_codes:
#         #     explain_result = subprocess.run(["rustc", "--explain", f"E{code}"],
#         #                                     capture_output=True, text=True, encoding="utf-8")
#         #     explanation = explain_result.stdout.strip()
#         #     issues.append(f"错误 E{code} 的详细说明:\n{explanation}")

#         # 添加总结性错误信息
#         summary_match = re.search(r'error: aborting due to (\d+) previous errors', clippy_output)
#         if summary_match:
#             # error_count = summary_match.group(1)
#             issues.append(f"总计: {error_count} 个错误")
#         return "\n\n".join(issues) if issues else "", issues

# def check_changelog(rust_code_lines, changelog):
#     # 正则表达式模式来匹配OriginalCode和FixedCode块
#     pattern = re.compile(r'(OriginalCode@.*?)(?=OriginalCode@|$)', re.DOTALL)
#     # 查找所有匹配的块
#     matches = pattern.findall(changelog)
#     # 提取为行号到代码的映射
#     rust_lines_map = {int(line.split(']', 1)[0][1:]): line.split(']', 1)[1].strip() for line in rust_code_lines}
#     applied_logpairs=[]
#     # 遍历每个匹配的块
#     for match in matches:
#         # 提取OriginalCode和FixedCode
#         original_code_match = re.search(r'OriginalCode@.*?:\n(.*?)(?=FixedCode@|$)', match, re.DOTALL)
#         fixed_code_match = re.search(r'FixedCode@.*?:\n(.*?)(?=OriginalCode@|$)', match, re.DOTALL)
        
#         if original_code_match and fixed_code_match:
#             fixed_code = fixed_code_match.group(1).strip()
#             original_code = original_code_match.group(1).strip()
#             # 提取为行号到代码的映射
#             original_lines = re.split(r'(?=\[\d+\])', original_code)
#             original_lines = [line.strip() for line in original_lines if line]
#             original_lines_map = {int(line.split(']', 1)[0][1:]): line.split(']', 1)[1].strip() for line in original_lines}

#             # 检查是否接受这条changelog（original code必须与原代码相同）
#             accept = True
#             for line_number, original_line in original_lines_map.items():
#                 if line_number not in rust_lines_map or original_line != rust_lines_map[line_number]:
#                     accept = False
#                     break
#             if accept:
#                 applied_logpairs.append((original_code, fixed_code))
#     return applied_logpairs
            

# api_conversion = api_agent.generate_response(
#         api_prompt.format(c_code = c_code)
#     )

# combined_syntax_input = convert_prompt.format(c_code=c_code, api_conversion=api_conversion)
# rust_code = syntax_agent.generate_response(combined_syntax_input)
# rust_code = extract_rust_code(rust_code)

# while True:
#     print("------------rust code------------")
#     print(rust_code)
#     print("静态分析")
#     analysis_result, issues = static_analysis(rust_code)

#     # analysis_result="stub"

#     if analysis_result:
#         print("静态分析结果：", analysis_result)
#         print("发现静态分析问题，生成fix log...")

#         block_1 = issues[0]
#         print(f"选择：{block_1}")

#         # 正则表达式，匹配双引号内的内容，并替换换行符为特殊标记
#         modified_str = re.sub(r'(".*?)(\n)(.*?")', lambda m: m.group(0).replace('\n', '___NEWLINE___'), rust_code)
#         # 按行分割字符串
#         lines = [line for line in modified_str.splitlines() if line]
#         # 恢复双引号内的特殊标记
#         lines = [line.replace('___NEWLINE___', '\n') for line in lines]
#         # 给每一行前加上行号
#         numbered_lines = [f"[{index + 1}] {line}" for index, line in enumerate(lines)]
#         # 将带有行号的行合并回一个字符串
#         numbered_rust_code = '\n'.join(numbered_lines)

#         fix_input = fix_prompt.format(error_block=block_1, code_snippets=numbered_rust_code)
#         debug_print("------------fix_input------------")
#         debug_print(fix_input)

#         fix_log = fix_agent.generate_response(fix_input)
#         print("修复日志：")
#         print(fix_log)

#         print("------------修复前------------")
#         print(numbered_rust_code)
#         # print(modified_str)

#         applied_logpairs = check_changelog(numbered_lines, fix_log)
#         print(f"经检查{len(applied_logpairs)}条changelog被采用，开始修复")
#         for (original_code, fixed_code) in applied_logpairs:
#             debug_print("-" * 40)
#             print("Original Code:")
#             print(original_code)
#             print("Fixed Code:")
#             print(fixed_code)
#             numbered_rust_code = numbered_rust_code.replace(original_code, fixed_code)
#             debug_print("------应用修复------")
#             debug_print(numbered_rust_code)
#         print("------------修复后------------")
#         print(numbered_rust_code)

#         # 正则表达式，匹配双引号内的内容，并替换换行符为特殊标记
#         modified_str = re.sub(r'(".*?)(\n)(.*?")', lambda m: m.group(0).replace('\n', '___NEWLINE___'), numbered_rust_code)
#         # 按行分割字符串
#         lines = [line for line in modified_str.splitlines() if line]
#         # 恢复双引号内的特殊标记
#         lines = [line.replace('___NEWLINE___', '\n') for line in lines]
#         # 去除行号
#         lines = [re.sub(r'^\[\w+\] ', '', line) for line in lines]
#         # 合并回代码
#         rust_code = '\n'.join(lines)
#     else:
#         print("静态分析未发现问题，进入下一阶段")
#         break

from new_prompts import *
c_code="aaa"
rust_code="bbb"
error_or_mismatch_info="ccc"
str1="""
The above Rust code is translated from C code, followed by the error message of Rust code runtime error.
### Original C Code:
#include <stdio.h>
void reverse_array(int arr[], int size) {
    int start = 0;
    int end = size - 1;
    while (start < end) {
        int temp = arr[start];
        arr[start] = arr[end];
        arr[end] = temp;
        start++;
        end--;
    }
}
int main() {
    int arr[] = {1, 2, 3, 4, 5};
    int size = sizeof(arr) / sizeof(arr[0]);
    reverse_array(arr, size);
    for (int i = 0; i < size; i++) {
        printf("%d ", arr[i]);
    }
    printf("\n");
    return 0;
}
### Translated Rust Code:
fn reverse_array(arr: &mut [i32]) {
    let len = arr.len();
    for i in 0..len / 2 {
        let temp = arr[i];
        arr[i] = arr[len - i];
        arr[len - i] = temp;
    }
}
fn main() {
    let mut arr = [1, 2, 3, 4, 5];
    reverse_array(&mut arr);
    for &item in &arr {
        print!("{} ", item);
    }
    println!();
}
### Error message:
thread 'main' panicked at test.rs:6:18:
index out of bounds: the len is 5 but the index is 5
### Instructions:
Analyze the above error and refer to the original C code to infer the cause of the error.
You need to locate the Rust code snippets that caused the error and provide the corresponding fixed Rust code snippets in a series of issues
Note that there may be multiple errors caused by the same error Rust code snippet.
Do not add comments or code that is not necessary to fix the error. 
Format Instruction:
Fix of each error Rust code snippet should be placed separately in an issue, noted that this error Rust code snippet may involve one or multiple errors.
Each issue should start with the summary of errors it is involved.
Then provide your speculated reasons for these errors, and give the Rust code snippet that caused the errors, followed by a fix code snippet.
Number of issues should not exceed 5.
output:
#Issue 1
Errors: Array out of bounds access.
Reason: When i is 0, arr [len-i] is actually arr [len], which exceeds the valid index range of the array (the valid index range of the array is 0 to len-1).
Error Code:
arr[i] = arr[len - i];
arr[len - i] = temp;
Fix Code:
arr[i] = arr[len - 1 - i];
arr[len - 1 - i] = temp;
"""
str="""
---
The above Rust code is translated from C code, followed by the error message of Rust code runtime error.
### Original C Code:
{c_code}
### Translated Rust Code
{rust_code}
### Error message:
{error_message}
### Instructions:
Analyze the above error and refer to the original C code to infer the cause of the error.
You need to locate the Rust code snippets that caused the error and provide the corresponding fixed Rust code snippets in a series of issues
Note that there may be multiple errors caused by the same error Rust code snippet.
Do not add comments or code that is not necessary to fix the error. 
Format Instruction:
Fix of each error Rust code snippet should be placed separately in an issue, noted that this error Rust code snippet may involve one or multiple errors.
Each issue should start with the summary of errors it is involved.
Then provide your speculated reasons for these errors, and give the Rust code snippet that caused the errors, followed by a fix code snippet.
Number of issues should not exceed 5.
output:
"""
fix_input = str.format(c_code=c_code, rust_code=rust_code, error_message=error_or_mismatch_info)
print(str1+fix_input)
    
