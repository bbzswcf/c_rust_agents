import json
import os
# 文件路径
metadata_file_path = '../tool/c_metadata.json'
target_file_path = '01-Primary/test-without-alloc'
if not os.path.exists(target_file_path):
    os.mkdir(target_file_path)

with open(metadata_file_path, 'r') as file:
    metadata = json.load(file)

for key, value in metadata.items():
    if "test" not in key:
        continue
    if "cpp" in key:
        continue
    file_content = []
    filename = key.split('/')[1]
    includes = value["includes"]
    functions = value["functions"]
    variables = value["variables"]
    head_info = value["head_info"]

    for item in includes:
        file_content.append(item["code"]+"\n")
    for item in head_info:
        file_content.append(item["code"]+"\n")
    for item in variables:
        file_content.append(item["code"]+"\n")
    for item in functions:
        file_content.append(item["code"]+"\n")
    print(filename)
    # print('\n'.join(file_content))
    
    with open(os.path.join(target_file_path, filename), "w") as file:
        file.writelines(file_content)
    # break
