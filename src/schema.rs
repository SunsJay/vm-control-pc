// @generated automatically by Diesel CLI.

diesel::table! {
    vmxq_status (id) {
        id -> Integer,
        name -> Text,
        has_deleted -> Nullable<Bool>,
        has_id -> Nullable<Bool>,
        has_5ma -> Nullable<Bool>,
        has_failed -> Nullable<Bool>,
        login_failed -> Nullable<Bool>,
        send_failed -> Nullable<Bool>,
        has_activated -> Nullable<Bool>,
        login_success -> Nullable<Bool>,
        send_test_success -> Nullable<Bool>,
        send_test_failed -> Nullable<Bool>,
        saohao_test_failed -> Nullable<Bool>,
        saohao_test_success -> Nullable<Bool>,
        silence_time -> Nullable<Integer>,
        available_time -> Nullable<Timestamp>,
        failed_time -> Nullable<Timestamp>,
        alive_time -> Nullable<Integer>,
        is_sending -> Nullable<Bool>,
        activated_time -> Nullable<Timestamp>,
        has_copy_vmxq -> Nullable<Bool>,
        has_copy_vmxl -> Nullable<Bool>,
        has_cleared -> Nullable<Bool>,
    }
}
