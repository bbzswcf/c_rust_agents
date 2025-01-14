import pytest
import os
import sys
import json

parent_dir = os.path.dirname(os.path.abspath(__file__))
sys.path.append(os.path.dirname(parent_dir))

from c_code_preprocess import (
    remove_file_comments,
    remove_header_guards,
    remove_cpp_guards,
    remove_copyright,
    process_alloc_testing,
    remove_test_includes,
    remove_debug_blocks,
    remove_alloc_test_blocks,
    extract_func_calls,
    preprocess,
    head_preprocess,
    code_preprocess
)

def test_remove_file_comments():
    # 测试单行注释
    code = """
    int main() {
        // This is a single line comment
        int x = 5; // inline comment
        return x;
    }
    """
    result = remove_file_comments(code)
    assert "// This is a single line comment" not in result
    assert "// inline comment" not in result
    assert "int x = 5;" in result

    # 测试多行注释
    code = """
    /* This is a
     * multi-line
     * comment */
    int main() {
        return 0;
    }
    """
    result = remove_file_comments(code)
    assert "/* This is a" not in result
    assert "multi-line" not in result
    assert "comment */" not in result
    assert "int main()" in result

    # 测试混合注释
    code = """
    /* Header comment */
    int func() {
        // Function comment
        return 1; /* inline multi-line */
    }
    """
    result = remove_file_comments(code)
    assert "/* Header comment */" not in result
    assert "// Function comment" not in result
    assert "/* inline multi-line */" not in result
    assert "return 1;" in result

    # 测试边界情况
    assert remove_file_comments("") == ""  # 空字符串
    assert remove_file_comments("/**/") == ""  # 仅注释
    assert remove_file_comments("code // comment\n") == "code"  # 行尾注释

def test_remove_header_guards():
    # 测试标准头文件guard
    code = """
#ifndef HEADER_H
#define HEADER_H

int function(void);
void test(void);

#endif
"""
    result = remove_header_guards(code)
    assert "#ifndef" not in result
    assert "#define" not in result
    assert "#endif" not in result
    assert "int function(void);" in result
    assert "void test(void);" in result

    # 测试不同格式的guard（带空格和缩进）
    code = """
    #ifndef  MY_GUARD_H
    #define  MY_GUARD_H    
        int x;
        int y;
    #endif
    """
    result = remove_header_guards(code)
    assert "#ifndef" not in result
    assert "#define" not in result
    assert "#endif" not in result
    assert "int x;" in result
    assert "int y;" in result

    # 测试没有guard的代码
    code = """
    int main() {
        return 0;
    }
    """
    result = remove_header_guards(code)
    assert result == code

    # 测试复杂内容
    code = """
#ifndef COMPLEX_H
#define COMPLEX_H

#include <stdio.h>
#define MAX 100

typedef struct {
    int data;
} Node;

#endif
"""
    result = remove_header_guards(code)
    assert "#ifndef COMPLEX_H" not in result
    assert "#define COMPLEX_H" not in result
    assert "#endif" not in result
    assert "#include <stdio.h>" in result
    assert "#define MAX 100" in result
    assert "typedef struct" in result

    # 测试边界情况
    assert remove_header_guards("") == ""
    assert remove_header_guards("#ifndef A\n#define A\n#endif") == ""

def test_remove_cpp_guards():
    # 测试基本C++ guard
    code = """
    #ifdef __cplusplus
    extern "C" {
    #endif
    
    void c_function(void);
    
    #ifdef __cplusplus
    }
    #endif
    """
    result = remove_cpp_guards(code)
    assert "#ifdef __cplusplus" not in result
    assert "extern \"C\"" not in result
    assert "void c_function(void);" in result

    # 测试不同格式的guard（带空格和缩进）
    code = """int before;
    #ifdef    __cplusplus
        extern "C" {
    #endif
    int middle;
    #ifdef    __cplusplus
        }
    #endif
    int after;"""
    result = remove_cpp_guards(code)
    assert "int before;" in result
    assert "int middle;" in result
    assert "int after;" in result
    assert "#ifdef" not in result
    assert "extern" not in result

    # 测试无guard的代码
    code = """
    void normal_function(void) {
        return;
    }
    """
    result = remove_cpp_guards(code)
    assert result.strip() == code.strip()

    # 测试边界情况
    assert remove_cpp_guards("") == ""
    assert remove_cpp_guards("#ifdef __cplusplus\n#endif\n") == ""
    
    # 测试多个guard块
    code = """
    #ifdef __cplusplus
    namespace test {
    #endif
    void func1(void);
    #ifdef __cplusplus
    }  // namespace
    #endif
    
    #ifdef __cplusplus
    extern "C" {
    #endif
    void func2(void);
    #ifdef __cplusplus
    }
    #endif
    """
    result = remove_cpp_guards(code)
    assert "void func1(void);" in result
    assert "void func2(void);" in result
    assert "namespace" not in result
    assert "#ifdef" not in result

def test_remove_copyright():
    # 测试标准版权声明
    code = """/* Copyright (c) 2024 Company Name */
    int main() {
        return 0;
    }"""
    result = remove_copyright(code)
    assert "Copyright" not in result
    assert "int main()" in result

    # 测试多行版权声明
    code = """/* 
     * Copyright (c) 2024
     * All rights reserved.
     */
    void test() {}"""
    result = remove_copyright(code)
    assert "Copyright" not in result
    assert "All rights reserved" not in result
    assert "void test()" in result

    # 测试不含版权声明的代码
    code = """/* Regular comment */
    void function() {}"""
    result = remove_copyright(code)
    assert result.strip() == code.strip()

    # 测试边界情况
    assert remove_copyright("") == ""
    assert remove_copyright("/* Copyright */") == ""
    
    # 测试复杂版权声明
    code = """/* 
     * Complex Copyright (c) 2020-2024
     * Company Name, Inc.
     * All rights reserved.
     * Some other text
     */
    #include <stdio.h>"""
    result = remove_copyright(code)
    assert "Copyright" not in result
    assert "#include <stdio.h>" in result

def test_process_alloc_testing():
    # 测试 ALLOC_TESTING 块移除
    code = """
    void test() {
        int x = 1;
    #ifdef ALLOC_TESTING
        malloc_test();
        free_test();
    #endif
        int y = 2;
    }"""
    result = process_alloc_testing(code)
    assert "#ifdef ALLOC_TESTING" not in result
    assert "malloc_test();" not in result
    assert "int x = 1;" in result
    assert "int y = 2;" in result

    # 测试 malloc/free 测试注释移除
    code = """
    void* ptr;
    /* malloc() / free() testing */
    ptr = malloc(10);
    """
    result = process_alloc_testing(code)
    assert "/* malloc() / free() testing */" not in result
    assert "void* ptr;" in result
    assert "ptr = malloc(10);" in result

    # 测试组合情况
    code = """
    #ifdef ALLOC_TESTING
    test_malloc();
    #endif
    /* malloc() / free() testing */
    void func() {
    #ifdef ALLOC_TESTING
        more_tests();
    #endif
    }"""
    result = process_alloc_testing(code)
    assert "#ifdef ALLOC_TESTING" not in result
    assert "/* malloc() / free() testing */" not in result
    assert "test_malloc();" not in result
    assert "more_tests();" not in result
    assert "void func() {" in result

    # 测试无相关代码
    code = """
    int main() {
        return 0;
    }"""
    result = process_alloc_testing(code)
    assert result.strip() == code.strip()

    # 测试边界情况
    assert process_alloc_testing("") == ""
    assert process_alloc_testing("#ifdef ALLOC_TESTING\n#endif\n") == ""
    assert process_alloc_testing("/* malloc() / free() testing */") == ""

def test_remove_test_includes():
    # 测试移除 framework.h
    code = """#include "framework.h"
    void test_func() {
        int x = 1;
    }"""
    result = remove_test_includes(code)
    assert '#include "framework.h"' not in result
    assert 'void test_func()' in result

    # 测试移除 alloc-testing.h
    code = """#include "alloc-testing.h"
    void test_alloc() {
        char* ptr = malloc(10);
    }"""
    result = remove_test_includes(code)
    assert '#include "alloc-testing.h"' not in result
    assert 'void test_alloc()' in result

    # 测试同时包含两种头文件
    code = """#include "framework.h"
    #include "alloc-testing.h"
    void test_both() {
        run_test();
    }"""
    result = remove_test_includes(code)
    assert '#include "framework.h"' not in result
    assert '#include "alloc-testing.h"' not in result
    assert 'void test_both()' in result

    # 测试无相关包含的代码
    code = """#include <stdio.h>
    void normal_func() {
        printf("test");
    }"""
    result = remove_test_includes(code)
    assert result.strip() == code.strip()

    # 测试边界情况
    assert remove_test_includes("") == ""
    assert remove_test_includes('#include "framework.h"\n') == ""
    assert remove_test_includes('#include "alloc-testing.h"\n') == ""

def test_remove_debug_blocks():
    # 测试基本调试块
    code = """
    void main() {
    #if 0
        printf("Debug info");
        log_debug();
    #endif
        return;
    }"""
    result = remove_debug_blocks(code)
    assert "#if 0" not in result
    assert "printf(\"Debug info\");" not in result
    assert "log_debug();" not in result
    assert "void main()" in result
    assert "return;" in result

    # 测试多个调试块
    code = """
    #if 0
    debug1();
    #endif
    int x = 1;
    #if 0
    debug2();
    #endif
    """
    result = remove_debug_blocks(code)
    assert "#if 0" not in result
    assert "debug1();" not in result
    assert "debug2();" not in result
    assert "int x = 1;" in result

    # 测试带缩进的调试块
    code = """void func() {
        int a = 1;
        #if 0
            debug_print();
            log_var(a);
        #endif
        int b = 2;
    }"""
    result = remove_debug_blocks(code)
    assert "debug_print();" not in result
    assert "log_var(a);" not in result
    assert "int a = 1;" in result
    assert "int b = 2;" in result

    # 测试无调试块的代码
    code = """
    int normal_function() {
        return 42;
    }"""
    result = remove_debug_blocks(code)
    assert result.strip() == code.strip()

    # 测试边界情况
    assert remove_debug_blocks("") == ""
    assert remove_debug_blocks("#if 0\ndebug();\n#endif") == "\n"

def test_remove_alloc_test_blocks():
    # 测试基本 alloc_test 块
    code = """void test_func() {
    alloc_test {
        test_malloc();
        test_free();
    }
}"""
    result = remove_alloc_test_blocks(code)
    assert "alloc_test" not in result
    assert "test_malloc();" not in result
    assert "test_free();" not in result
    assert "void test_func() {" in result
    assert result.endswith("}")

    # 测试多个大括号的情况
    code = """void test_complex() {
    int x = {1};
    alloc_test {
        if (1) {
            test_malloc();
        }
    }
}"""
    result = remove_alloc_test_blocks(code)
    assert "alloc_test" not in result
    assert "int x = {1};" in result
    assert "test_malloc();" not in result
    assert result.endswith("}")

    # 测试无 alloc_test 块的代码
    code = """void normal_func() {
    int x = 1;
    return;
}"""
    result = remove_alloc_test_blocks(code)
    assert result == code

    # 测试特殊格式的 alloc_test
    code = """void test() {
        alloc_test    {
            test_something();
        }
    }"""
    result = remove_alloc_test_blocks(code)
    assert "alloc_test" not in result
    assert "test_something();" not in result
    assert "void test() {" in result
    assert result.endswith("}")

    # 测试边界情况
    assert remove_alloc_test_blocks("") == ""
    assert remove_alloc_test_blocks("alloc_test") == "alloc_test"  # 无大括号
    assert remove_alloc_test_blocks("alloc_test {") == "alloc_test {"  # 无结束大括号

def test_extract_func_calls():
    """
    Test function to verify extract_func_calls behavior
    """
    # Test function pointers as parameters
    code = """
    void test_avl_tree_new(void)
{
	AVLTree *tree;

	tree = avl_tree_new((AVLTreeCompareFunc) int_compare);

	assert(tree != NULL);
	assert(avl_tree_root_node(tree) == NULL);
	assert(avl_tree_num_entries(tree) == 0);

	avl_tree_free(tree);

	/* Test out of memory scenario */

	alloc_test_set_limit(0);

	tree = avl_tree_new((AVLTreeCompareFunc) int_compare);

	assert(tree == NULL);

}
    """
    calls = extract_func_calls(code)
    assert calls == ['avl_tree_new', 'assert', 'avl_tree_root_node', 'avl_tree_num_entries', 'avl_tree_free', 'alloc_test_set_limit', 'int_compare', 'tree']

def test_preprocess_complex():
    # 测试复杂的嵌套代码
    code = """/* Copyright 2024 */
    #ifndef COMPLEX_H
    #define COMPLEX_H
    
    /* File header comment */
    #ifdef __cplusplus
    extern "C" {
    #endif
    
    /* Multi-line 
     * comment with
     * multiple lines
     */
    typedef struct {
        int data;  // Inline comment
    } Node;
    
    #ifdef __cplusplus
    }  // extern "C"
    #endif
    
    #endif // COMPLEX_H
    """
    
    result = preprocess(code)
    print(result)
    
    # 验证所有注释被移除
    assert "Copyright" not in result
    assert "File header" not in result
    assert "Multi-line" not in result
    assert "Inline comment" not in result
    
    # 验证所有守卫被移除
    assert "#ifndef COMPLEX_H" not in result
    assert "#ifndef INNER_H" not in result
    assert "#ifdef __cplusplus" not in result
    assert "#endif" not in result
    
    # 验证代码结构保持
    assert "typedef struct {" in result
    assert "int data;" in result
    assert "} Node;" in result

    # 测试极端嵌套情况
    code = """/* Header */
    #ifndef OUTER_H
    #define OUTER_H
    /* Comment 1 */
    #ifdef __cplusplus
    extern "C" {
    #endif
    /* Comment 2 */
    /* Comment 3 */
    void func(); // Function
    #ifdef __cplusplus
    }
    #endif
    #endif
    """
    
    result = preprocess(code)
    assert "void func();" in result
    assert "Header" not in result
    assert "Comment" not in result
    assert "#ifndef" not in result
    assert "#ifdef" not in result
    assert "#endif" not in result

def test_head_preprocess_complex():
    # 测试典型的头文件结构
    header_code = """/* Copyright notice
     * Multiple lines
     */
     
    #ifndef MY_HEADER_H_
    #define MY_HEADER_H_
    
    /* Standard includes */
    #include <stdio.h>
    #include <stdlib.h>
    
    #ifdef __cplusplus
    extern "C" {
    #endif
    
    /* Function declarations */
    typedef struct MyStruct {
        int data;    /* Data member */
        void* ptr;   // Pointer member
    } MyStruct;
    
    /* API Functions */
    void init_struct(MyStruct* s);  /* Initialize structure */
    int process_data(const MyStruct* s);  // Process data
    
    #ifdef __cplusplus
    }  /* extern "C" */
    #endif
    
    #endif /* MY_HEADER_H_ */
    """
    
    result = head_preprocess(header_code)
    
    # 验证注释移除
    assert "Copyright notice" not in result
    assert "Standard includes" not in result
    assert "Function declarations" not in result
    assert "Data member" not in result
    assert "Initialize structure" not in result
    
    # 验证守卫移除
    assert "#ifndef MY_HEADER_H_" not in result
    assert "#define MY_HEADER_H_" not in result
    assert "#ifdef __cplusplus" not in result
    assert "extern \"C\"" not in result
    assert "#endif" not in result
    
    # 验证代码结构保持
    assert "#include <stdio.h>" in result
    assert "#include <stdlib.h>" in result
    assert "typedef struct MyStruct {" in result
    assert "int data;" in result
    assert "void* ptr;" in result
    assert "} MyStruct;" in result
    assert "void init_struct(MyStruct* s);" in result
    assert "int process_data(const MyStruct* s);" in result

    # 测试嵌套条件编译
    header_code = """
    #ifndef NESTED_H
    #define NESTED_H
    
    #ifdef __cplusplus
    extern "C" {
    #endif
    
    typedef void (*callback_t)(void*);  /* Function pointer */
    
    #ifdef __cplusplus
    }
    #endif
    #endif
    """
    
    result = head_preprocess(header_code)
    assert "typedef void (*callback_t)(void*);" in result
    assert "#ifdef _WIN32" not in result
    assert "#ifndef NESTED_H" not in result
    assert "extern \"C\"" not in result
    assert "Windows-specific code" not in result
    assert "Unix-specific code" not in result

def test_code_preprocess():
    # 直接对已执行的'tool/c_metadata_all.json'中的数据进行验证
    if not os.path.exists('tool/c_metadata_all.json'):
        code_preprocess("./Input/01-Primary")
    
    # 读取生成的元数据文件
    with open('tool/c_metadata_all.json', 'r', encoding='utf-8') as f:
        metadata = json.load(f)
    
    # 验证元数据结构
    assert isinstance(metadata, dict)
    
    # 验证src文件处理
    rb_tree_file = os.path.join('src', 'rb-tree.c')
    if rb_tree_file in metadata:
        rb_tree_data = metadata[rb_tree_file]
        # 验证基本字段
        assert 'functions' in rb_tree_data
        assert 'includes' in rb_tree_data
        assert 'variables' in rb_tree_data
        assert 'func_signatures' in rb_tree_data
        
        # 验证函数依赖关系
        for func in rb_tree_data['functions']:
            if func['name'] == 'rb_tree_insert_case3':
                # 验证特殊处理：移除对rb_tree_insert_case1的依赖
                depend_funcs = [d['name'] for d in func['depend_funcs']]
                assert 'rb_tree_insert_case1' not in depend_funcs
    
    # 验证测试文件处理
    test_files = [f for f in metadata.keys() if f.startswith('test-')]
    for test_file in test_files:
        test_data = metadata[test_file]
        # 验证测试文件特性
        assert 'functions' in test_data
        # 验证main函数被排除
        assert not any(f['name'] == 'main' for f in test_data['functions'])
        # 验证out_of_memory测试被排除
        assert not any('out_of_memory' in f['signature'] for f in test_data['functions'])
    
    # 验证头文件处理
    header_files = [f for f in metadata.keys() if f.endswith('.c') and not f.startswith('test-')]
    for header_file in header_files:
        header_data = metadata[header_file]
        # 验证公共函数签名
        assert 'public_func_signatures' in header_data
        assert 'private_func_signatures' in header_data
        # 验证头文件信息
        assert 'head_info' in header_data

if __name__ == '__main__':
    # -v 表示显示详细输出
    # -s 表示显示print输出
    pytest.main(['-v', '-s', __file__, '--cache-clear'])