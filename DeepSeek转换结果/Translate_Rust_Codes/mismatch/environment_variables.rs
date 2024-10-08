use std::env;
use std::path::PathBuf;

fn main() {
    // 修改: 统一使用PathBuf来处理路径,并使用display方法来输出路径
    let home_dir = if let Some(home) = env::var_os("HOME") {
        PathBuf::from(home)
    } else if let Some(userprofile) = env::var_os("USERPROFILE") {
        PathBuf::from(userprofile)
    } else {
        println!("Environment variable HOME or USERPROFILE is not set.");
        return;
    };

    // 修改: 使用display方法来输出路径,确保跨平台兼容性
    println!("{}", home_dir.display());
}