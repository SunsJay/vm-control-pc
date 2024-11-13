use crate::utils::env::init_env;
use crate::utils::log::init_log;
use crate::utils::time::start_time;

pub mod config;

pub mod model;

pub mod service;

pub mod utils;

pub mod global;



pub fn init() {

    init_log().expect("初始化日志失败");
    log::info!("> 项目初始化");
    start_time();
    init_env().ok();
    // let database_url = env::var("DATABASE_URL").expect("DATABASE_URL 没有在 .env 文件里设置");

}

