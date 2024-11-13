use log::{error, info};
use serde_json::Value;
use crate::model::VmxqStatus;

pub fn parse_json(content: &str) -> Option<VmxqStatus> {
    match serde_json::from_str::<VmxqStatus>(content) {
        Ok(json_data) => {
            info!("解析JSON数据成功: {}", json_data);
            Some(json_data)
        },
        Err(err) => {
            error!("解析JSON数据失败: {}", err);
            None
        }
    }
}