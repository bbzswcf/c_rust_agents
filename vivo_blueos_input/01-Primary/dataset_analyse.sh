cnt_array=()
while read -r file; do
    # 检查文件是否是 C 文件
    if [[ "$file" == *.c ]]; then
        # 生成 AST 并计算 FunctionDecl 的数量
        clang -Xclang -ast-dump -fsyntax-only "$file" > ast_dump.txt
        cnt=$(grep -c 'FunctionDecl' ast_dump.txt)
        cnt=$((cnt / 2))

        # 将 cnt 添加到数组中
        cnt_array+=("$cnt")
    fi
done < <(find "$1" -type f)

# 计算最大值、最小值和平均值
max=${cnt_array[0]}
min=${cnt_array[0]}
sum=0
count=0

for cnt in "${cnt_array[@]}"; do
    # 更新最大值
    if (( cnt > max )); then
        max=$cnt
    fi

    # 更新最小值
    if (( cnt < min )); then
        min=$cnt
    fi

    # 累加总和
    sum=$((sum + cnt))
    count=$((count + 1))
done
echo $cnt_array
# 计算平均值
if (( count > 0 )); then
    average=$((sum / count))
else
    average=0
fi

# 输出结果
echo "Max: $max"
echo "Min: $min"
echo "Average: $average"