/*
 * @Date: 2024-11-13 16:47:06
 * @LastEditors: sunsjay sunsjay0806@gmail.com
 * @LastEditTime: 2024-11-21 20:39:39
 * @FilePath: /vm-control/src/utils/json.rs
 * @Description: 
 */
use log::error;

use crate::model::VmxqStatusVar;

pub fn parse_json(content: &str) -> Option<VmxqStatusVar> {
    match serde_json::from_str::<VmxqStatusVar>(content) {
        Ok(json_data) => {
            // info!("解析JSON数据成功: {:?}", json_data);
            Some(json_data)
        },
        Err(err) => {
            error!("解析JSON数据失败: {}", err);
            None
        }
    }
}