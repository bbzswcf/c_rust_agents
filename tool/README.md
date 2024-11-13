## 多agent C-Rust项目级代码翻译（拆分成函数进行翻译）
### Idea（只针对vivo数据集）
测试文件中有不同的测试函数，每个测试函数有不同的依赖函数
找到测试函数的所有依赖函数，对依赖函数进行翻译
再对测试函数进行翻译
构建运行环境，运行翻译后的测试函数
如果通过，则测试函数和依赖函数翻译成功，写入文件
### 文件结构
- 01primary_analyzer.py: 对数据集进行初步分析
- blue_os_c_rust.py: 在原有c_rust.py框架上进行修改
- Agent_prompt_simple.py: 不同agent的prompt
- input_prompt.py: 变量、结构体翻译的prompt
- c_code_preprocess.py: 预处理代码
- c_metadata.json: 预处理后的数据
- tree_sitter_analyzer.py: 使用tree-sitter分析数据
