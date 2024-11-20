/*
 * @Date: 2024-11-13 15:25:48
 * @LastEditors: sunsjay sunsjay0806@gmail.com
 * @LastEditTime: 2024-11-20 22:22:42
 * @FilePath: /vm-control/src/service/vm.rs
 * @Description: 
 */
use std::{env, fs};
use diesel::{ RunQueryDsl, SelectableHelper, SqliteConnection};
use log::{error, info};
use crate::schema::vmxq_status::dsl::*;
use diesel::prelude::*;
use crate::{schema::vmxq_status, utils::json::parse_json, VmxqStatus, VmxqStatusNew};


pub fn get_vm_info(connection: &mut SqliteConnection, suffix: &str)  {
    let dir_path = env::var("ZI_PAN").expect("ZI_PAN 没有在 .env 文件里设置");

    if let Ok(entries) = fs::read_dir(dir_path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_dir() {
                    info!("目录: {}", path.display()); 
                    let vm_name = format!("{}", path.display());
                    let vmxq_new = create_vm(connection, &vm_name);

                    query_vmxq_status(connection, vmxq_new.id);
                    if let Ok(sub_entries) = fs::read_dir(&path) {
                        for sub_entry in sub_entries {
                            let sub_path = sub_entry.expect("获取子盘的状态文件错误").path();

                            if let Some(ext) = sub_path.extension() {
                                if ext == suffix {
                                    // info!("找到.vmxqstatus文件: {}", sub_path.display());
                                    if let Ok(content) = fs::read_to_string(&sub_path) {
                                        let vmxq_status_var = parse_json(&content);

                                        if let Some(vmxq_status_var) = vmxq_status_var {
                                            update_vmxq_status(connection,  &VmxqStatus {
                                                id: vmxq_new.id,
                                                name: vm_name.clone(),
                                                has_5ma: vmxq_status_var.has_5ma,
                                                has_deleted: vmxq_status_var.has_deleted,
                                                has_id: vmxq_status_var.has_id,
                                                has_failed: vmxq_status_var.has_failed,
                                                login_failed: vmxq_status_var.login_failed,
                                                send_failed: vmxq_status_var.send_failed,
                                                has_activated: vmxq_status_var.has_activated,
                                                login_success: vmxq_status_var.login_success,
                                                send_test_success: vmxq_status_var.send_test_success,
                                                send_test_failed: vmxq_status_var.send_test_failed,
                                                saohao_test_failed: vmxq_status_var.saohao_test_failed,
                                                saohao_test_success: vmxq_status_var.saohao_test_success,
                                                silence_time: vmxq_status_var.silence_time,
                                                available_time: vmxq_status_var.available_time,
                                                failed_time: vmxq_status_var.failed_time,
                                                alive_time: vmxq_status_var.alive_time,
                                                is_sending: vmxq_status_var.is_sending,
                                                activated_time: vmxq_status_var.activated_time,
                                                has_copy_vmxq: vmxq_status_var.has_copy_vmxq,
                                                has_copy_vmxl: vmxq_status_var.has_copy_vmxl,
                                                has_cleared: vmxq_status_var.has_cleared,
                    
                                                
                                            });
                                        }
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

pub fn update_vmxq_status(connection: &mut SqliteConnection, vmxq_status_new: &VmxqStatus)  {
    diesel::update(vmxq_status::table.find(vmxq_status_new.id))
        .set(vmxq_status_new)
        .execute(connection)
        .expect("更新vmxq失败");
}