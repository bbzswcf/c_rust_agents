from tree_sitter import Parser
import subprocess
import os

def build_languages():
    # 确保目录存在
    for directory in ['vendor', 'build']:
        if not os.path.exists(directory):
            os.makedirs(directory)
    
    # 克隆 tree-sitter-rust 仓库
    if not os.path.exists('vendor/tree-sitter-rust'):
        subprocess.run(['git', 'clone', 'https://github.com/tree-sitter/tree-sitter-rust.git', 'vendor/tree-sitter-rust'])

    # 在 Windows 系统上，共享库扩展名应该是 .dll
    output_file = 'build/rust.so'
    
    # 编译语言库，包含 scanner.c
    subprocess.run(['gcc',
                   '-o', output_file,
                   '-shared',
                   '-fPIC',
                   'vendor/tree-sitter-rust/src/parser.c',
                   'vendor/tree-sitter-rust/src/scanner.c',  # 添加 scanner.c
                   '-I', 'vendor/tree-sitter-rust/src'])     # 添加头文件路径

if __name__ == '__main__':
    build_languages()