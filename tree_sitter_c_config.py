import os
import platform
from tree_sitter import Language, Parser

# 获取当前文件的绝对路径
current_directory = os.getcwd()
ext = '.dll' if platform.system() == 'Windows' else '.so'

TREE_SITTER_PATH = os.path.join(current_directory, 'tool', 'tree_sitter', 'build', 'c' + ext)
TREE_SITTER_PATH_RUST = os.path.join(current_directory, 'tool', 'tree_sitter', 'build', 'rust' + ext)

C_LANGUAGE = Language(TREE_SITTER_PATH, 'c')
c_parser = Parser()
c_parser.set_language(C_LANGUAGE)

RUST_LANGUAGE = Language(TREE_SITTER_PATH_RUST, 'rust')
rust_parser = Parser()
rust_parser.set_language(RUST_LANGUAGE)