"""rust代码整合脚本"""
import json
import os
import shutil
import subprocess

from tree_sitter_analyzer import analyze_directory

metadata_path = 'tool/c_metadata.json'
output_path = 'Output'
project_name = 'demo'

def load_metadata(path: str):
    """加载 metadata.json 文件"""
    return json.load(open(path, 'r', encoding='utf-8'))

def write_file(file_path: str, content: str):
    """写入文件"""
    os.makedirs(os.path.dirname(file_path), exist_ok=True)
    with open(file_path, 'w') as f:
        f.write(content)

def insert_file(file_path: str, content: str):
    """插入文件"""
    os.makedirs(os.path.dirname(file_path), exist_ok=True)
    with open(file_path, 'a') as f:
        f.write(content)

def create_project_structure():
    """创建项目基本结构"""
    # 使用 subprocess 运行 cargo new --lib 命令
    subprocess.run(["cargo", "new", "--lib", project_name], check=True, cwd=output_path)
    project_path = os.path.join(output_path, project_name)
    lib_path = os.path.join(project_path, "src/lib.rs")
    os.makedirs(os.path.join(project_path, 'tests'), exist_ok=True)
    cargo_content = """libc = \"0.2\""""
    insert_file(os.path.join(project_path, "Cargo.toml"), cargo_content)
    

def generate_rust_files(metadata: dict):
    project_path = os.path.join(output_path, project_name)
    lib_path = os.path.join(project_path, "src/lib.rs")
    write_file(lib_path, "pub mod translation_utils;\n")

    dependencies = analyze_directory(metadata)

    for file_path, file_info in metadata.items():
        file_name = os.path.splitext(os.path.basename(file_path.replace('-', '_')))[0]
        rust_file_path = ''
        if file_path.startswith('src'):
            insert_file(lib_path, f"pub mod {file_name};\n")
            rust_file_path = os.path.join(project_path, 'src', file_name + '.rs')
        else:
            rust_file_path = os.path.join(project_path, 'tests', file_name + '.rs')
            for depend_file in dependencies[file_path]:
                if depend_file.startswith('src'):
                    rust_mod_name = os.path.basename(depend_file.replace('-', '_'))
                    insert_file(rust_file_path, f"use {project_name}::{rust_mod_name}::*;\n")
        
        insert_file(rust_file_path, file_info['rust_items'])
        for func_info in file_info['functions']:
            insert_file(rust_file_path, func_info['rust_code'])
    # Copy translation_utils directory to project src directory
    translation_utils_src = os.path.join(output_path, 'primary/src/translation_utils/')
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

    

def generate_project(metadata_path: str):
    """生成整个项目"""
    metadata = load_metadata(metadata_path)
    create_project_structure()
    generate_rust_files(metadata)


if __name__ == "__main__":
    generate_project(metadata_path)
    