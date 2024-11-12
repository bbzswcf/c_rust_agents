# 删除所有注释
# 使用：./remove_comments.sh target_dir_name
find "$1" -type f | while read -r file; do
    # 删注释
    sed -i ':a; /\/*/,/\*\//{N; s/\/\*.*\*\///g; ba}; s/\/\/.*//g' "$file"
    # 删空白行
    sed -i '/^\s*$/d' "$file"
    echo "Processing file: $file"
done