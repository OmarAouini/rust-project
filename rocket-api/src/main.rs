//!
//! This crate is the REST API binary with api endpoints
//! Author: Omar Aouini
//! 20/02/2022
//!

mod apiresponse;

#[macro_use] extern crate rocket;

use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::serde_json::json;
use rocket::{serde, State};
use rocket::serde::json::Json;
use crate::apiresponse::ApiResponse;

#[get("/")]
fn world() -> &'static str {
    "Hello, world!"
}

#[post("/count", data="<company>")]
fn add(pool: &State<core::sqlx::MySqlPool>, company: Json<core::company::Company>) -> ApiResponse {
    use core::traits::Crud;
    let mut codod = company.into_inner();
    codod.email = "test".to_string();
    ApiResponse{ json: json!(codod), status: Status::Created}
}

#[get("/count")]
fn count(pool: &State<core::sqlx::MySqlPool>) -> ApiResponse {
    use core::traits::Crud;
    let all = core::company::Company::findAll(&pool).unwrap();
    format!("Number of visits: {:?}", all);
    let comp = core::company::Company{
        id: 0,
        name: "".to_string(),
        vat_code: "".to_string(),
        address: "".to_string(),
        email: "".to_string(),
        phone_number: "".to_string(),
        projects: vec![]
    };
    ApiResponse{status: Status::Ok, json: json!(comp)}
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
        .manage(pool.clone())
        .mount("/hello", routes![world, count, add])
        .launch()
        .await;
}