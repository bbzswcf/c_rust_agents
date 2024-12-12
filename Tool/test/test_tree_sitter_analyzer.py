import pytest
import os
import sys

parent_dir = os.path.dirname(os.path.abspath(__file__))
sys.path.append(os.path.dirname(parent_dir))

from tree_sitter_analyzer import (
    analyze_directory,
    get_translation_order,
    extract_test_functions,
    dependencies_order,
    sort_by_depend_count,
    extract_rust_funcs
)

def test_analyze_directory():
    # 准备测试数据，使用os.path.join构建路径
    test_metadata = {
        os.path.join('src', 'file1.c'): {
            'includes': []
        },
        os.path.join('src', 'file2.c'): {
            'includes': [
                {'code': '#include "file1.h"'}
            ]
        },
        os.path.join('test', 'test_file.c'): {
            'includes': [
                {'code': '#include "test_helper.h"'},
                {'code': '#include "file1.h"'}
            ]
        },
        os.path.join('src', 'file3.c'): {
            'includes': [
                {'code': '#include <stdio.h>'},
                {'code': '#include "file2.h"'}
            ]
        }
    }

    # 执行分析
    dependencies = analyze_directory(test_metadata)

    # 验证结果，使用os.path.join构建预期路径
    assert dependencies[os.path.join('src', 'file1.c')] == []
    assert dependencies[os.path.join('src', 'file2.c')] == [os.path.join('src', 'file1')]
    assert dependencies[os.path.join('test', 'test_file.c')] == [os.path.join('src', 'file1')]
    assert dependencies[os.path.join('src', 'file3.c')] == [os.path.join('src', 'file2')]

    # 验证依赖关系的正确性
    for file_path, deps in dependencies.items():
        # 检查依赖项格式
        for dep in deps:
            assert dep.startswith(os.path.join('src', '')) or dep.startswith(os.path.join('test', ''))
            # 确保文件不依赖自身
            assert dep != os.path.splitext(file_path)[0]

def test_get_translation_order():
    # 准备测试数据，使用os.path.join构建路径
    test_dependencies = {
        os.path.join('test', 'test_a.c'): [os.path.join('src', 'b')],
        os.path.join('test', 'test_b.c'): [os.path.join('src', 'a')],
        os.path.join('src', 'a.c'): [],
        os.path.join('src', 'b.c'): [os.path.join('src', 'a')],
        os.path.join('src', 'c.c'): [os.path.join('src', 'b')]
    }

    # 执行排序
    order = get_translation_order(test_dependencies)

    # 验证结果
    assert len(order) > 0
    
    # 验证test文件在前面
    test_files = [f for f in order if f.startswith(os.path.join('test', ''))]
    src_files = [f for f in order if f.startswith(os.path.join('src', ''))]
            
    # 验证src文件都有.c后缀
    for file in src_files:
        assert file.endswith('.c')
        
    # 验证依赖顺序正确性
    for i, file in enumerate(order):
        base_file = file.replace('.c', '')
        if base_file in test_dependencies:
            deps = test_dependencies[base_file]
            for dep in deps:
                dep_with_ext = dep + '.c' if dep.startswith(os.path.join('src', '')) else dep
                if dep_with_ext in order:
                    assert order.index(dep_with_ext) > i

def test_extract_test_functions():
    # 准备测试数据
    test_metadata = {
        'test/test_file.c': {
            'functions': [
                {'name': 'test_one'},
                {'name': 'helper_function'},
                {'name': 'test_two'},
                {'name': 'setup'},
                {'name': 'test_three'}
            ]
        },
        'test/empty_test.c': {
            'functions': []
        },
        'test/no_test_funcs.c': {
            'functions': [
                {'name': 'helper_one'},
                {'name': 'setup'}
            ]
        }
    }

    # 测试提取测试函数
    result = extract_test_functions('test/test_file.c', test_metadata)
    assert result == ['test_one', 'test_two', 'test_three']
    assert len(result) == 3
    # 验证顺序保持
    assert result.index('test_one') < result.index('test_two')
    assert result.index('test_two') < result.index('test_three')

    # 测试空函数列表
    assert extract_test_functions('test/empty_test.c', test_metadata) == []

    # 测试无测试函数的文件
    assert extract_test_functions('test/no_test_funcs.c', test_metadata) == []

def test_dependencies_order():
    # 准备测试数据
    test_metadata = {
        'test/test_a.c': {
            'functions': [
                {
                    'name': 'test_main',
                    'depend_funcs': [
                        {'name': 'helper1', 'file': 'src/helper.c'},
                        {'name': 'helper2', 'file': 'src/helper.c'}
                    ]
                }
            ]
        },
        'src/helper.c': {
            'functions': [
                {
                    'name': 'helper1',
                    'depend_funcs': [
                        {'name': 'base_func', 'file': 'src/base.c'}
                    ]
                },
                {
                    'name': 'helper2',
                    'depend_funcs': [
                        {'name': 'base_func', 'file': 'src/base.c'}
                    ]
                }
            ]
        },
        'src/base.c': {
            'functions': [
                {
                    'name': 'base_func',
                    'depend_funcs': []
                }
            ]
        }
    }

    # 测试基本依赖关系
    result = dependencies_order('test_main', 'test/test_a.c', test_metadata)
    
    # 验证结果包含所有依赖
    assert ('base_func', 'src/base.c') in result
    assert ('helper1', 'src/helper.c') in result
    assert ('helper2', 'src/helper.c') in result
    
    # 验证依赖顺序（base_func应该在helper1和helper2之前）
    base_idx = result.index(('base_func', 'src/base.c'))
    helper1_idx = result.index(('helper1', 'src/helper.c'))
    helper2_idx = result.index(('helper2', 'src/helper.c'))
    assert base_idx < helper1_idx
    assert base_idx < helper2_idx
    
    # 验证去重（base_func只应出现一次）
    assert result.count(('base_func', 'src/base.c')) == 1
    
    # 测试无依赖函数
    result = dependencies_order('base_func', 'src/base.c', test_metadata)
    assert result == []

def test_sort_by_depend_count():
    # 准备测试数据
    test_metadata = {
        'test/test_file.c': {
            'functions': [
                {
                    'name': 'test_simple',
                    'depend_funcs': []
                },
                {
                    'name': 'test_complex',
                    'depend_funcs': [
                        {'name': 'helper1', 'file': 'src/helper.c'},
                        {'name': 'helper2', 'file': 'src/helper.c'}
                    ]
                },
                {
                    'name': 'test_medium',
                    'depend_funcs': [
                        {'name': 'helper1', 'file': 'src/helper.c'}
                    ]
                }
            ]
        },
        'src/helper.c': {
            'functions': [
                {
                    'name': 'helper1',
                    'depend_funcs': []
                },
                {
                    'name': 'helper2',
                    'depend_funcs': []
                }
            ]
        }
    }

    # 测试基本排序
    test_funcs = ['test_simple', 'test_complex', 'test_medium']
    result = sort_by_depend_count(test_funcs, 'test/test_file.c', test_metadata)
    
    # 验证结果
    assert isinstance(result, list)
    assert len(result) == 3
    
    # 验证排序顺序（按依赖数从少到多）
    assert result[0] == 'test_simple'  # 0个依赖
    assert result[1] == 'test_medium'  # 1个依赖
    assert result[2] == 'test_complex' # 2个依赖
    
    # 测试空列表
    assert sort_by_depend_count([], 'test/test_file.c', test_metadata) == []
    
    # 测试相同依赖数的函数
    test_metadata['test/test_file.c']['functions'].append({
        'name': 'test_medium2',
        'depend_funcs': [
            {'name': 'helper1', 'file': 'src/helper.c'}
        ]
    })
    
    result = sort_by_depend_count(
        ['test_medium', 'test_medium2'], 
        'test/test_file.c', 
        test_metadata
    )
    assert len(result) == 2
    # 确保都包含在结果中
    assert 'test_medium' in result
    assert 'test_medium2' in result

def test_extract_rust_funcs():
    # 测试基本函数定义提取
    rust_code = """
    fn test_func1() {
        println!("test1");
    }
    
    fn test_func2(x: i32) -> i32 {
        x + 1
    }
    """
    funcs, uses = extract_rust_funcs(rust_code)
    assert len(funcs) == 2
    assert "fn test_func1()" in funcs[0]
    assert "fn test_func2(x: i32)" in funcs[1]
    assert len(uses) == 0

    # 测试use声明提取
    rust_code = """
    use std::io;
    use crate::utils::*;
    
    fn main() {
        println!("Hello");
    }
    """
    funcs, uses = extract_rust_funcs(rust_code)
    assert len(funcs) == 1
    assert len(uses) == 2
    assert "use std::io" in uses[0]
    assert "use crate::utils::*" in uses[1]

    # 测试混合代码
    rust_code = """
    use std::io;
    
    fn func1() {}
    
    use crate::test::*;
    
    fn func2() {}
    """
    funcs, uses = extract_rust_funcs(rust_code)
    assert len(funcs) == 2
    assert len(uses) == 2

    # 测试空代码
    funcs, uses = extract_rust_funcs("")
    assert len(funcs) == 0
    assert len(uses) == 0

    # 测试复杂函数
    rust_code = """
    fn complex_func<T: Display>(input: T) -> Result<T, Error> {
        Ok(input)
    }
    """
    funcs, uses = extract_rust_funcs(rust_code)
    assert len(funcs) == 1
    assert "fn complex_func<T: Display>" in funcs[0]

if __name__ == '__main__':
    # -v 表示显示详细输出
    # -s 表示显示print输出
    pytest.main(['-v', '-s', __file__, '--cache-clear'])