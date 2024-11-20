/*
 * @Date: 2024-11-13 15:25:48
 * @LastEditors: sunsjay sunsjay0806@gmail.com
 * @LastEditTime: 2024-11-15 22:11:36
 * @FilePath: /vm-control/src/service/vm.rs
 * @Description: 
 */
use std::{env, fs};
use diesel::{ RunQueryDsl, SelectableHelper, SqliteConnection};
use log::{error, info};
use crate::schema::vmxq_status::dsl::*;
use diesel::prelude::*;
use crate::{schema::vmxq_status, utils::json::parse_json, VmxqStatus, VmxqStatusNew};


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
                                        parse_json(&content);
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

pub fn create_vm(conn: &mut SqliteConnection, vm_name: &str) -> VmxqStatus {


    let vmxq_new = VmxqStatusNew {  name: vm_name.to_string() };
    diesel::insert_into(vmxq_status::table).values(&vmxq_new).returning(VmxqStatus::as_returning())
    .get_result(conn).expect("Error creating vm")

}

pub fn query_vmxq_status(connection: &mut SqliteConnection, target_id: i32)  {

    let results = vmxq_status
        .filter(id.eq(target_id))
        .limit(1)
        .select(VmxqStatus::as_select())
        .load(connection)
        .expect("查询失败");
    println!("{:?}", results);
    
}