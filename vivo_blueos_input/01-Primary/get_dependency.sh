TARGET_DIR="no_comments"
file_list=(
    "framework"
    "alloc-testing"
    "test-alloc-testing"
    "arraylist"
    "test-arraylist"
    "avl-tree"
    "test-avl-tree"
    "binary-heap"
    "test-binary-heap"
    "binomial-heap"
    "test-binomial-heap"
    "bloom-filter"
    "test-bloom-filter"
    "compare-int"
    "test-compare-int"
    "compare-pointer"
    "test-compare-pointer"
    "compare-string"
    "test-compare-string"
    "hash-int"
    "test-hash-int"
    "hash-pointer"
    "test-hash-pointer"
    "hash-string"
    "test-hash-string"
    "hash-table"
    "test-hash-table"
    "list"
    "test-list"
    "queue"
    "test-queue"
    "rb-tree"
    "test-rb-tree"
    "set"
    "test-set"
    "slist"
    "test-slist"
    "sortedarray"
    "test-sortedarray"
    "trie"
    "test-trie"
)
echo -n > dependency.txt
find "$TARGET_DIR" -type f | sort | while read -r file; do
    base_name=$(basename "$file" .c)
    echo "$base_name": >> dependency.txt
    grep '#include "' "$file" | sed -E 's/#include "(.*).h"/\1/' \
    | grep -v "$base_name" \
    | paste -sd ' ' | awk '{printf "\t%s\n", $0}' >> dependency.txt
done
# 过滤自定义头文件名
