use std::fmt;

use diesel::{Queryable, Selectable};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::vmxq_status)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[serde(rename_all = "camelCase")]
pub struct VmxqStatus {
    pub id: i32,
    pub name: String,
    pub has_deleted: Option<bool>,
    pub has_id: Option<bool>,
    pub has_5ma: Option<bool>,
    pub has_failed: Option<bool>,
    pub login_failed: Option<bool>,
    pub send_failed: Option<bool>,
    pub has_activated: Option<bool>,
    pub login_success: Option<bool>,
    pub send_test_success: Option<bool>,
    pub send_test_failed: Option<bool>,
    pub saohao_test_failed: Option<bool>,
    pub saohao_test_success: Option<bool>,
    pub silence_time: Option<i32>,
    pub available_time: Option<String>,
    pub failed_time: Option<String>,
    pub alive_time: Option<i32>,
    pub is_sending: Option<bool>,
    pub activated_time: Option<String>,
    pub has_copy_vmxq: Option<bool>,
    pub has_copy_vmxl: Option<bool>,
    pub has_cleared: Option<bool>,
}


#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Insertable)]

#[diesel(table_name = crate::schema::vmxq_status)]
pub struct VmxqStatusNew {
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VmxqStatusVar {
    has_deleted: Option<bool>,
    has_id: Option<bool>,
    has_5ma: Option<bool>,
    has_failed: Option<bool>,
    login_failed: Option<bool>,
    send_failed: Option<bool>,
    has_activated: Option<bool>,
    login_success: Option<bool>,
    send_test_success: Option<bool>,
    send_test_failed: Option<bool>,
    saohao_test_failed: Option<bool>,
    saohao_test_success: Option<bool>,
    silence_time: Option<i32>,
    available_time: Option<String>,
    failed_time: Option<String>,
    alive_time: Option<i32>,
    is_sending: Option<bool>,
    activated_time: Option<String>,
    has_copy_vmxq: Option<bool>,
    has_copy_vmxl: Option<bool>,
    has_cleared: Option<bool>,
}


impl fmt::Display for VmxqStatusVar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "VmxqStatus:\n")?;

        if let Some(activated_time) = &self.activated_time {
            write!(f, "Activated Time: {}\n", activated_time)?;
        }

        if let Some(alive_time) = &self.alive_time {
            write!(f, "Alive Time: {}\n", alive_time)?;
        }

        if let Some(available_time) = &self.available_time {
            write!(f, "Available Time: {}\n", available_time)?;
        }

        if let Some(failed_time) = &self.failed_time {
            write!(f, "Failed Time: {}\n", failed_time)?;
        }

        if let Some(has_5ma) = &self.has_5ma {
            write!(f, "Has 5ma: {}\n", has_5ma)?;
        }

        if let Some(has_activated) = &self.has_activated {
            write!(f, "Has Activated: {}\n", has_activated)?;
        }

        if let Some(has_cleared) = &self.has_cleared {
            write!(f, "Has Cleared: {}\n", has_cleared)?;
        }

        if let Some(has_copy_vmxl) = &self.has_copy_vmxl {
            write!(f, "Has Copy Vmxl: {}\n", has_copy_vmxl)?;
        }

        if let Some(has_copy_vmxq) = &self.has_copy_vmxq {
            write!(f, "Has Copy Vmxq: {}\n", has_copy_vmxq)?;
        }

        if let Some(has_deleted) = &self.has_deleted {
            write!(f, "Has Deleted: {}\n", has_deleted)?;
        }

        if let Some(has_failed) = &self.has_failed {
            write!(f, "Has Failed: {}\n", has_failed)?;
        }

        if let Some(has_id) = &self.has_id {
            write!(f, "Has ID: {}\n", has_id)?;
        }

        if let Some(is_sending) = &self.is_sending {
            write!(f, "Is Sending: {}\n", is_sending)?;
        }

        if let Some(login_failed) = &self.login_failed {
            write!(f, "Login Failed: {}\n", login_failed)?;
        }

        if let Some(login_success) = &self.login_success {
            write!(f, "Login Success: {}\n", login_success)?;
        }

        if let Some(saohao_test_failed) = &self.saohao_test_failed {
            write!(f, "Saohao Test Failed: {}\n", saohao_test_failed)?;
        }

        if let Some(saohao_test_success) = &self.saohao_test_success {
            write!(f, "Saohao Test Success: {}\n", saohao_test_success)?;
        }

        if let Some(send_failed) = &self.send_failed {
            write!(f, "Send Failed: {}\n", send_failed)?;
        }

        if let Some(send_test_failed) = &self.send_test_failed {
            write!(f, "Send Test Failed: {}\n", send_test_failed)?;
        }

        if let Some(send_test_success) = &self.send_test_success {
            write!(f, "Send Test Success: {}\n", send_test_success)?;
        }

        if let Some(silence_time) = &self.silence_time {
            write!(f, "Silence Time: {}\n", silence_time)?;
        }

        Ok(())
    }
}