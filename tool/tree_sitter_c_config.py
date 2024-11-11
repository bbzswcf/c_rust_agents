from tree_sitter import Language, Parser

# TREE_SITTER_PATH = 'D:/code/python/tree-sitter/build/my-languages.so'
TREE_SITTER_PATH = "/mnt/sda/xc/VarSearch/search-2.0/build/my-languages.so"
C_LANGUAGE = Language(TREE_SITTER_PATH, 'c')
c_parser = Parser()
c_parser.set_language(C_LANGUAGE)