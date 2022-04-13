//!
//! This crate is the REST API binary with api endpoints
//! Author: Omar Aouini
//! 20/02/2022
//!

mod apiresponse;
mod api;

#[macro_use] extern crate rocket;
use crate::apiresponse::ApiResponse;

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
    let pool : core::sqlx::MySqlPool =
        core::db::connect_db_pool("mysql://root:root@localhost/mysite", 1, 5).await;

    rocket::build()
        .manage(pool)
        .mount("/", routes![api::health])
        .mount("/companies",
               routes![
                   api::company::add,
                   api::company::all,
                   api::company::find,
                   api::company::update,
                   api::company::delete
               ])
        .mount("/employees", routes![])
        .mount("/projects", routes![])
        .mount("/tasks", routes![])
        .launch()
        .await;
}