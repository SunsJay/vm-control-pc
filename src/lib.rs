use crate::db::establish_connection;
use crate::model::*;
use crate::service::vm::get_vm_info;
use crate::utils::env::init_env;
use crate::utils::log::init_log;
use crate::utils::time::start_time;
use diesel::prelude::*;
use crate::schema::vmxq_status::dsl::*;
pub mod config;

pub mod model;
pub mod global;
pub mod service;

pub mod utils;
pub mod schema;

pub mod db;


pub fn init() {


    init_log().expect("初始化日志失败");
    log::info!("项目初始化");
    start_time();
    init_env().ok();



    get_vm_info("vmxqstatus");

    let connection = &mut establish_connection();
    let results = vmxq_status
        .filter(id.eq(1))
        .limit(1)
        .select(VmxqStatus::as_select())
        .load(connection)
        .expect("查询失败");
    println!("{:?}", results);
    println!("Displaying {} posts", results.len());

}

