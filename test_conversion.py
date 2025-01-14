import os
import pytest
from input_prompt import *
from Agent_prompt import *
import shutil
from blue_os_c_rust import (
    convert_c_funcs_to_rust,
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
def test_basic_conversion(sample_c_code):
    """测试基本C代码转换"""
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
    assert "Box::new" in rust_code # 检查是否使用 Box 分配
def fresh():
    with open(os.path.join(test_output_dir, "src/lib.rs"), "w") as f:
        f.write(f"""
#![allow(semicolon_in_expressions_from_macros)]  // 允许宏中的表达式中有分号                             
#![allow(clippy::mut_from_ref)]
pub(crate) mod translation_utils;
use crate::translation_utils::*;
""")
    
if __name__ == '__main__':
    create_temp_rust_project(test_output_dir, "test_conversion")
    # -v 表示显示详细输出
    # -s 表示显示print输出
    pytest.main(['-v', '-s', __file__, '--cache-clear'])
    # Clean up test directory
    if os.path.exists(test_output_dir):
        shutil.rmtree(test_output_dir)