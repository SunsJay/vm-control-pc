use std::fmt;
use serde::Deserialize;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct VmxqStatus {
    activated_time: Option<String>,
    alive_time: Option<u32>,
    available_time: Option<String>,
    failed_time: Option<String>,
    has_5ma: Option<bool>,
    has_activated: Option<bool>,
    has_cleared: Option<bool>,
    has_copy_vmxl: Option<bool>,
    has_copy_vmxq: Option<bool>,
    has_deleted: Option<bool>,
    has_failed: Option<bool>,
    has_id: Option<bool>,
    is_sending: Option<bool>,
    login_failed: Option<bool>,
    login_success: Option<bool>,
    saohao_test_failed: Option<bool>,
    saohao_test_success: Option<bool>,
    send_failed: Option<bool>,
    send_test_failed: Option<bool>,
    send_test_success: Option<bool>,
    silence_time: Option<u32>,
}


impl fmt::Display for VmxqStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "activated_time: {}, alive_time: {}, available_time: {}, failed_time: {}, has_5ma: {}, has_activated: {}, has_cleared: {}, has_copy_vmxl: {}, has_copy_vmxq: {}, has_deleted: {}, has_failed: {}, has_id: {}, is_sending: {}, login_failed: {}, login_success: {}, saohao_test_failed: {}, saohao_test_success: {}, send_failed: {}, send_test_failed: {}, send_test_success: {}, silence_time: {}",
               self.activated_time, self.alive_time, self.available_time, self.failed_time, self.has_5ma, self.has_activated, self.has_cleared, self.has_copy_vmxl, self.has_copy_vmxq, self.has_deleted, self.has_failed, self.has_id, self.is_sending, self.login_failed, self.login_success, self.saohao_test_failed, self.saohao_test_success, self.send_failed, self.send_test_failed, self.send_test_success, self.silence_time)
    }
}