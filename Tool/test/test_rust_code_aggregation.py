import pytest
import os
import sys
import json
import shutil

parent_dir = os.path.dirname(os.path.abspath(__file__))
sys.path.append(os.path.dirname(parent_dir))

from rust_code_aggregation import create_project_structure, generate_rust_files

metadata_path = 'Tool/c_metadata_all.json'
output_path = 'Output'
project_name = 'test_demo'

@pytest.fixture
def temp_project_dir():
    """Get expected project directory path"""
    return os.path.join(output_path, project_name)

def test_create_project_structure(temp_project_dir):
    """Test if project structure is created correctly"""
    if not os.path.exists(temp_project_dir):
        create_project_structure(output_path, project_name)
    
    # Check if directories were created
    assert os.path.exists(temp_project_dir)
    assert os.path.exists(os.path.join(temp_project_dir, 'src'))
    
    # Check if Cargo.toml is created with correct content
    cargo_path = os.path.join(temp_project_dir, "Cargo.toml")
    assert os.path.exists(cargo_path)
    
    with open(cargo_path, 'r') as f:
        content = f.read()
        assert 'libc = "0.2"' in content
    
    # Check if lib.rs file is created
    lib_path = os.path.join(temp_project_dir, "src/lib.rs")
    assert os.path.exists(lib_path)
    # Clean up the created project directory after the test
    shutil.rmtree(temp_project_dir)

def test_generate_rust_files(temp_project_dir):
    # 读取实际metadata
    with open(metadata_path) as f:
        metadata = json.load(f)
    
    # 执行函数
    if not os.path.exists(temp_project_dir):
        create_project_structure(output_path, project_name)
        generate_rust_files(metadata, output_path, project_name)
    
    project_path = temp_project_dir
    
    # 验证项目结构
    assert os.path.exists(project_path)
    assert os.path.exists(os.path.join(project_path, 'src'))
    assert os.path.exists(os.path.join(project_path, 'src/lib.rs'))
    
    # 验证translation_utils复制
    assert os.path.exists(os.path.join(project_path, 'src/translation_utils'))
    
    # 验证生成的Rust文件
    with open(os.path.join(project_path, 'src/lib.rs')) as f:
        content = f.read()
        assert 'pub(crate) mod translation_utils' in content
        
    # 检查至少一个源文件被正确生成
    sample_file = next(iter(metadata.keys()))
    sample_name = os.path.splitext(os.path.basename(sample_file.replace('-', '_')))[0]
    rust_file = os.path.join(project_path, 'src', f'{sample_name}.rs')
    
    assert os.path.exists(rust_file)
    with open(rust_file) as f:
        content = f.read()
        assert 'use crate::translation_utils::*' in content
    
    # Clean up the created project directory after the test
    shutil.rmtree(temp_project_dir)

if __name__ == '__main__':
    # -v 表示显示详细输出
    # -s 表示显示print输出
    pytest.main(['-v', '-s', __file__, '--cache-clear'])
