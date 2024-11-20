#!/bin/bash

# 遍历当前文件夹下的所有文件
for file in *; do
    # 检查文件是否为普通文件
    if [[ -f "$file" ]]; then
        # 在文件末尾添加代码
        echo -e "\nint main(int argc, char *argv[])\n{\n\treturn 0;\n}" >> "$file"
        echo "Added main function to $file"
    fi
done
