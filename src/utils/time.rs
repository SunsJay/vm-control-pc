use chrono::{Local, Timelike};

pub fn start_time() {
    let local_time = Local::now().with_nanosecond(0).unwrap();

    log::info!("当前时间: {}" ,local_time);
}