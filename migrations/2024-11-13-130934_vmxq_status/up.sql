-- Your SQL goes here

CREATE TABLE vmxq_status (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    name VARCHAR NOT NULL,
    has_deleted BOOLEAN,
    has_id BOOLEAN,
    has_5ma BOOLEAN,
    has_failed BOOLEAN,
    login_failed BOOLEAN,
    send_failed BOOLEAN,
    has_activated BOOLEAN,
    login_success BOOLEAN,
    send_test_success BOOLEAN,
    send_test_failed BOOLEAN,
    saohao_test_failed BOOLEAN,
    saohao_test_success BOOLEAN,
    silence_time INTEGER,
    available_time TIMESTAMP,
    failed_time TIMESTAMP,
    alive_time INTEGER,
    is_sending BOOLEAN,
    activated_time TIMESTAMP,
    has_copy_vmxq BOOLEAN,
    has_copy_vmxl BOOLEAN,
    has_cleared BOOLEAN
);