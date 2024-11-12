use std::env;
use crate::utils::env::init_env;

pub mod config;

pub mod model;

pub mod service;

pub mod utils;

pub mod global;



pub fn init() {
    env_logger::init();
    log::info!("初始化");
    init_env().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL 没有在 .env 文件里设置");

    print!("{:?}", database_url);
}

