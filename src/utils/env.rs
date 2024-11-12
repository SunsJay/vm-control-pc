#![allow(unused_variables)]
#![allow(unused_assignments)]
use std::env;
use std::path::PathBuf;

use dotenv::dotenv;

pub fn init_env() -> dotenv::Result<PathBuf> {

    let mut env_file_path = String::new();

    // 根据操作系统类型设置不同的路径
    #[cfg(target_os = "windows")]
    {
        env_file_path = "C:\\Program Files\\vm".to_string();
    }

    #[cfg(target_os = "macos")]
    {
        env_file_path = "/Users/sunsjay/Desktop".to_string();
    }

    // 设置当前工作目录
    env::set_current_dir(&env_file_path).unwrap();

    // 加载.env文件中的环境变量
    dotenv()
}