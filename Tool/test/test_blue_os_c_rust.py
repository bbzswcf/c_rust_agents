import os
import sys
import shutil
import pytest

parent_dir = os.path.dirname(os.path.abspath(__file__))
sys.path.append(os.path.dirname(parent_dir))

from input_prompt import *
from Agent_prompt import *
from blue_os_c_rust import (
    extract_rust_code,
    sanitize_string,
    convert_c_funcs_to_rust,
    convert_c_initialization_to_rust,
    create_temp_rust_project
)

test_output_dir = "./Output/test_conversion"

@pytest.fixture
def sample_c_code():
    """基本的C代码样例"""
    return """
    ArrayList *arraylist_new(unsigned int length)
{
	ArrayList *new_arraylist;

	if (length <= 0) {
		length = 16;
	}

	new_arraylist = (ArrayList *) malloc(sizeof(ArrayList));

	if (new_arraylist == NULL) {
		return NULL;
	}

	new_arraylist->_alloced = length;
	new_arraylist->length = 0;

	new_arraylist->data = malloc(length * sizeof(ArrayListValue));

	if (new_arraylist->data == NULL) {
		free(new_arraylist);
		return NULL;
	}

	return new_arraylist;
}
    """

def fresh():
    with open(os.path.join(test_output_dir, "src/lib.rs"), "w") as f:
        f.write(f"""
#![allow(semicolon_in_expressions_from_macros)]  // 允许宏中的表达式中有分号                             
#![allow(clippy::mut_from_ref)]
pub(crate) mod translation_utils;
use crate::translation_utils::*;
""")

def test_extract_rust_code():
    # 测试正常Rust代码块提取
    review_text = """Here's the code:
    ```rust
    fn main() {
        println!("Hello");  // Print greeting
    }
    ```
    """
    result = extract_rust_code(review_text)
    assert "fn main()" in result
    assert 'println!("Hello")' in result
    assert "// Print greeting" not in result

    # 测试空输入
    assert extract_rust_code("") == ""
    assert extract_rust_code(None) == ""

    # 测试无代码块的文本
    text_without_code = "Just some regular text without code blocks"
    assert extract_rust_code(text_without_code) == text_without_code.strip()

    # 测试多个代码块 - 应只提取第一个
    multiple_blocks = """
    First block:
    ```rust
    fn first() {}
    ```
    Second block:
    ```rust
    fn second() {}
    ```
    """
    result = extract_rust_code(multiple_blocks)
    assert "fn first()" in result
    assert "fn second()" not in result

    # 测试复杂注释情况
    complex_code = """```rust
    fn test() {
        let x = 1;  // Inline comment
        // Full line comment
        let y = 2;  // Another comment
        /* Block comment */
    }
    ```"""
    result = extract_rust_code(complex_code)
    assert "// Inline comment" not in result
    assert "// Full line comment" not in result
    assert "// Another comment" not in result
    assert "let x = 1;" in result
    assert "let y = 2;" in result
    assert "/* Block comment */" in result  # 块注释应保留

def test_sanitize_string():
    # 测试ASCII字符
    assert sanitize_string("Hello World") == "Hello World"
    assert sanitize_string("abc123") == "abc123"
    
    # 测试中文字符
    assert sanitize_string("你好世界") == ""
    assert sanitize_string("Hello世界") == "Hello"
    
    # 测试特殊字符
    assert sanitize_string("café") == "cafe"  # é -> e
    assert sanitize_string("über") == "uber"  # ü -> u
    assert sanitize_string("piñata") == "pinata"  # ñ -> n
    
    # 测试组合字符
    assert sanitize_string("à la carte") == "a la carte"  # à -> a
    assert sanitize_string("résumé") == "resume"  # é -> e
    
    # 测试特殊情况
    assert sanitize_string("") == ""  # 空字符串
    assert sanitize_string("hello\u0301") == "hello"  # Unicode组合字符
    
    # 测试混合字符串
    complex_string = "Hello你好Café世界"
    assert sanitize_string(complex_string) == "HelloCafe"

def test_basic_conversion(sample_c_code):
    """测试基本C代码转换"""
    if not os.path.exists(test_output_dir):
        create_temp_rust_project(test_output_dir, "test_conversion")

    rust_file = os.path.join(test_output_dir, "src/lib.rs")
    fresh()
    
    metadata = {
        "test.c": {
            "functions": [{
                    "name": "arraylist_new",
                "code": sample_c_code,
                "rust_signature": "",
                "depend_funcs": []
            }],
            "rust_items": "",
            "includes": [],
            "head_info": [],
            "variables": []
        }
    }
    
    success, rust_code = convert_c_funcs_to_rust(
        "test.c",
        "arraylist_new",
        metadata["test.c"]["functions"][0],
        "",
        rust_file,
        test_output_dir,
        metadata,
        Syntax_prompt_rag,
        Syntax_example_input,
        Syntax_example_output
    )
    
    assert success
    assert "fn arraylist_new" in rust_code  # 检查函数名
    assert "Option<Box<ArrayList>>" in rust_code # 检查返回类型
    assert "malloc" not in rust_code # 确保没有 C malloc

    # Clean up test directory
    if os.path.exists(test_output_dir):
        shutil.rmtree(test_output_dir)

def test_convert_c_initialization_to_rust():
    if not os.path.exists(test_output_dir):
        create_temp_rust_project(test_output_dir, "test_conversion")

    rust_file = os.path.join(test_output_dir, "src/lib.rs")
    fresh()

    # 准备测试数据
    metadata = {
        'src/test.c': {
            'head_info': [
                {'code': 'typedef struct { int x; } TestStruct;'}
            ],
            'variables': [
                {'code': 'static int test_var = 0;'}
            ]
        },
        'test/test_file.c': {
            'head_info': [
                {'code': '#define TEST_VALUE 100'}
            ],
            'variables': [
                {'code': 'static TestStruct test_struct;'}
            ]
        }
    }

    # 测试普通源文件转换
    result = convert_c_initialization_to_rust(
        'src/test.c',
        rust_file,
        test_output_dir,
        metadata
    )
    assert "struct TestStruct" in result
    assert "static test_var" in result
    fresh()

    # 测试测试文件转换 
    result = convert_c_initialization_to_rust(
        'test/test_file.c',
        rust_file,
        test_output_dir,
        metadata
    )
    assert "test_struct" in result
    assert "TEST_VALUE" in result

    # 测试空输入
    empty_metadata = {'empty.c': {'head_info': [], 'variables': []}}
    result = convert_c_initialization_to_rust(
        'empty.c',
        rust_file,
        test_output_dir,
        empty_metadata
    )
    assert result == ''

    # Clean up
    if os.path.exists(test_output_dir):
        shutil.rmtree(test_output_dir)

if __name__ == '__main__':
    # -v 表示显示详细输出
    # -s 表示显示print输出
    pytest.main(['-v', '-s', __file__, '--cache-clear'])
