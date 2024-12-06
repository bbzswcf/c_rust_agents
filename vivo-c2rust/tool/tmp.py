import os
import subprocess

PROJ_PATH = "../primary"
def cargo_test():
    current_directory = os.getcwd()
    os.chdir(f"{PROJ_PATH}")
    # 运行 cargo test 命令
    env = os.environ.copy()
    env["RUSTFLAGS"] = "-Awarnings"
    try:
        result = subprocess.run(
            ["cargo", "test", "--", "--test-threads=1"],
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            text=True,
            env=env,
            timeout=5
        )
        os.chdir(current_directory)
        if result.returncode == 0:
            print("所有测试通过！")
            return (True, "")
        else:
            print("测试未通过！")
            error_msg = result.stderr
            print(error_msg)
            return (False, error_msg)
    except subprocess.TimeoutExpired:
        print("超时")
        os.chdir(current_directory)
        return (False, "Timeout error, Please ensure that your program does not cause errors or infinite loops.")

    # 检查命令的返回码
    

if __name__ == "__main__":
    cargo_test()


