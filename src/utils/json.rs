use log::{error, info};
use serde_json::Value;
use crate::model::VmxqStatus;

pub fn parse_json(content: &str) -> Option<VmxqStatus> {
    if let Ok(json_data) = serde_json::from_str::<VmxqStatus>(content) {
        info!("解析JSON数据成功: {}", json_data);
        Some(json_data)
    } else {
        error!("解析JSON数据失败");
        None
    }
}