import subprocess
import os
import platform

def build_languages():
    # 根据操作系统确定共享库扩展名
    ext = '.dll' if platform.system() == 'Windows' else '.so'
    
    # 克隆语言仓库
    repos = {
        'rust': 'https://github.com/tree-sitter/tree-sitter-rust.git',
        'c': 'https://github.com/tree-sitter/tree-sitter-c.git'
    }
    
    for lang, repo in repos.items():
        vendor_path = f'tool/tree_sitter/vendor/tree-sitter-{lang}'
        # 编译语言库
        output_file = f'tool/tree_sitter/build/{lang}{ext}'

        if os.path.exists(output_file):
            continue
        
        compile_args = [
            'gcc',
            '-o', output_file,
            '-shared',
            '-fPIC',
            f'{vendor_path}/src/parser.c',
        ]
        
        # Rust 需要额外的 scanner.c
        if lang == 'rust':
            compile_args.append(f'{vendor_path}/src/scanner.c')
            
        compile_args.extend(['-I', f'{vendor_path}/src'])
        
        subprocess.run(compile_args)

if __name__ == '__main__':
    build_languages()