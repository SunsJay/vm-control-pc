use std::{env, fs};
use log::{error, info};

pub fn get_vm_info(suffix: &str)  {
    let dir_path = env::var("ZI_PAN").expect("ZI_PAN 没有在 .env 文件里设置");

    if let Ok(entries) = fs::read_dir(dir_path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_dir() {
                    info!("目录: {}", path.display());

                    if let Ok(sub_entries) = fs::read_dir(&path) {
                        for sub_entry in sub_entries {
                            let sub_path = sub_entry.expect("获取子盘的状态文件错误").path();

                            if let Some(ext) = sub_path.extension() {
                                if ext == suffix {
                                    // info!("找到.vmxqstatus文件: {}", sub_path.display());
                                    if let Ok(content) = fs::read_to_string(&sub_path) {
                                        info!("文件内容 {}: \n{}", sub_path.display(), content);
                                    } else {
                                        error!("读取文件错误: {}", sub_path.display());
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    } else {
        error!("读取子盘目录失败")
    }
}