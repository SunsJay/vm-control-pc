use log::{error, info};

use crate::model::VmxqStatusVar;

pub fn parse_json(content: &str) -> Option<VmxqStatusVar> {
    match serde_json::from_str::<VmxqStatusVar>(content) {
        Ok(json_data) => {
            info!("解析JSON数据成功: {:?}", json_data);
            Some(json_data)
        },
        Err(err) => {
            error!("解析JSON数据失败: {}", err);
            None
        }
    }
}