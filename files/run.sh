# 构建c和rust解析器
/bin/python3 tool/tree_sitter/tree_sitter_build.py

# 执行翻译程序
/bin/python3 tool/blue_os_c_rust.py

/bin/python3 tool/rust_code_aggregation.py