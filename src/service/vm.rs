use std::{env, fs};
use log::{error, info};

pub fn get_vm_info()  {
    let dir_path = env::var("ZI_PAN").expect("ZI_PAN 没有在 .env 文件里设置");

    if let Ok(entries) = fs::read_dir(dir_path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_dir() {
                    info!("目录: {}", path.display());
                }
            }
        }
    } else {
        error!("读取子盘目录失败")
    }
}