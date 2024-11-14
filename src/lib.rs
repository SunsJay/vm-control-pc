/*
 * @Date: 2024-11-12 19:07:07
 * @LastEditors: sunsjay sunsjay0806@gmail.com
 * @LastEditTime: 2024-11-14 22:34:05
 * @FilePath: /vm-control/src/lib.rs
 * @Description:
 */
use crate::db::establish_connection;
use crate::model::*;

use crate::service::vm::get_vm_info;
use crate::utils::env::init_env;
use crate::utils::log::init_log;
use crate::utils::time::start_time;

use service::vm::{create_vm, query_vmxq_status};
pub mod config;

pub mod global;
pub mod model;
pub mod service;

pub mod schema;
pub mod utils;

pub mod db;

pub fn init() {
    
    init_log().expect("初始化日志失败");
    log::info!("项目初始化");
    start_time();
    init_env().ok();

    let connection = &mut establish_connection();
    
    get_vm_info("vmxqstatus");


    let vm_name: String = "1234567".to_string();

    let vmxq_new = create_vm(connection, &vm_name);

    query_vmxq_status(connection, vmxq_new.id);
}
