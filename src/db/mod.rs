/*
 * @Date: 2024-11-13 20:34:49
 * @LastEditors: sunsjay sunsjay0806@gmail.com
 * @LastEditTime: 2024-11-20 20:21:10
 * @FilePath: /vm-control/src/db/mod.rs
 * @Description:
 */
use diesel::{prelude::*, r2d2};
use lazy_static::lazy_static;

use diesel::r2d2::{ConnectionManager, Pool};
use diesel::SqliteConnection;
use std::env;

pub fn establish_connection() -> SqliteConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub type SqlitePool = Pool<ConnectionManager<SqliteConnection>>;

lazy_static! {
    pub static ref DBPOOL: SqlitePool = {
        let pool = establish_pool();
        pool
    };
}

pub fn establish_pool() -> SqlitePool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    r2d2::Pool::new(manager).unwrap()
}
