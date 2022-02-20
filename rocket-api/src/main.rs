//!
//! This crate is the REST API binary with api endpoints
//! Author: Omar Aouini
//! 20/02/2022
//!

#[macro_use] extern crate rocket;

use rocket::serde::json::Json;
use rocket::State;

#[get("/")]
fn world() -> &'static str {
    "Hello, world!"
}

#[get("/count")]
fn count(pool: &State<core::sqlx::MySqlPool>) -> Json<core::company::Company> {
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
    Json(comp)
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
        .mount("/hello", routes![world, count])
        .launch()
        .await;
}