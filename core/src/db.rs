//!
//! This module contains functions for database connections
//!
//! Author: Omar Aouini
//!
//! 20/02/2022
//!

use sqlx::{MySql, MySqlPool, Pool};
use sqlx::mysql::MySqlPoolOptions;

/// Connect to a Mysql Connection pool
pub async fn connect_db_pool() -> MySqlPool {
    let pool = MySqlPoolOptions::new()
        .min_connections(1)
        .max_connections(5)
        .connect("mysql://root:root@localhost/mysite").await;
    if let Err(err) = pool {
        panic!("error connection to {:?}", "mysql://root:root@localhost/mysite")
    }
    return pool.unwrap()
}