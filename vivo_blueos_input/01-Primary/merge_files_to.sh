# 将src和test的文件都复制到目标文件夹
# 合并.c和.h文件
# 使用：./merge_files_to.sh target_dir_name
src_path="initial_input/src"
test_path="initial_input/test"

if [ ! -d "$1" ]; then
  mkdir -p "$1"
fi

cp -i "$test_path"/* "$1"
cp -i "$src_path"/* "$1"
cur_pwd="$(pwd)"
pushd "$1" > /dev/null
# 遍历文件名列表
while read -r name; do
    # 合并
    cat "${name}.c" >> "${name}.h"
    # 删.h文件
    rm "${name}.c"
    mv "${name}.h" "${name}.c"
    # 删空白行
    sed -i '/^\s*$/d' "${name}.c"
done < "$cur_pwd"/filename_list
popd > /dev/null
