//!
//! This crate is the REST API binary with api endpoints
//! Author: Omar Aouini
//! 20/02/2022
//!

mod apiresponse;
mod api;

#[macro_use] extern crate rocket;

use rocket::http::Status;
use rocket::serde::json::serde_json::json;
use rocket::{Config, State};
use rocket::serde::json::Json;
use crate::apiresponse::ApiResponse;

#[get("/health")]
fn health() -> Status {
    Status::Ok
}

/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = my_crate::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
#[rocket::main]
async fn main() {
    //db
    let mut pool : core::sqlx::MySqlPool = core::db::connect_db_pool().await;

    rocket::build()
        .manage(pool)
        .mount("/",
               routes![health])
        .mount("/companies",
               routes![api::company::add,api::company::count,])
        .mount("/employees", routes![])
        .launch()
        .await;
}