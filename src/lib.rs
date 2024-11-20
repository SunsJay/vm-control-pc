/*
 * @Date: 2024-11-12 19:07:07
 * @LastEditors: sunsjay sunsjay0806@gmail.com
 * @LastEditTime: 2024-11-20 21:51:13
 * @FilePath: /vm-control/src/lib.rs
 * @Description:
 */

use crate::model::*;

use crate::service::vm::get_vm_info;
use crate::utils::env::init_env;
use crate::utils::log::init_log;
use crate::utils::time::start_time;

use db::DBPOOL;
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

    let connection = &mut DBPOOL.get().unwrap();
    
    get_vm_info(connection,"vmxqstatus");

}
