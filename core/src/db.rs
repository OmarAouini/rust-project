//!
//! This module contains functions for database connections
//!
//! Author: Omar Aouini
//!
//! 20/02/2022
//!

use sqlx::MySqlPool;
use sqlx::mysql::MySqlPoolOptions;

/// Connect to a Mysql Connection pool
pub async fn connect_db_pool(connection_string :&str, min_conn: u32, max_conn: u32) -> MySqlPool {
    let pool = MySqlPoolOptions::new()
        .min_connections(min_conn)
        .max_connections(max_conn)
        .connect(connection_string).await;
    if let Err(err) = pool {
        panic!("error connection to {:?}, err: {:?}",connection_string, err)
    }
    return pool.unwrap()
}