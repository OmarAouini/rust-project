use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::serde::json::serde_json::json;
use rocket::State;
use crate::ApiResponse;
use crate::apiresponse::ErrorJson;
use core::traits::Crud;

#[get("/")]
pub fn all(pool: &State<core::sqlx::MySqlPool>) -> ApiResponse {
    match core::company::Company::find_all(pool.inner()) {
        Ok(v) => ApiResponse{ json: json!(v), status: Status::Ok},
        Err(err) => ApiResponse{ json: json!(ErrorJson{message: err}), status: Status::InternalServerError}
    }
}

#[get("/<id>")]
pub fn find(pool: &State<core::sqlx::MySqlPool>, id :i32) -> ApiResponse {
    match core::company::Company::find(pool.inner(), &id) {
        Ok(v) => ApiResponse{ json: json!(v), status: Status::Ok},
        Err(err) => ApiResponse{ json: json!(ErrorJson{message: err}), status: Status::InternalServerError}
    }
}

#[post("/", data="<company>")]
pub fn add(pool: &State<core::sqlx::MySqlPool>, company: Json<core::company::Company>) -> ApiResponse {
    match core::company::Company::add(pool.inner(), &company.into_inner()) {
        Ok(v) => ApiResponse{ json: json!(v), status: Status::Created},
        Err(err) => ApiResponse{ json: json!(ErrorJson{message: err}), status: Status::InternalServerError}
    }
}

#[put("/", data="<company>")]
pub fn update(pool: &State<core::sqlx::MySqlPool>, company: Json<core::company::Company>) -> ApiResponse {
    match core::company::Company::update(pool.inner(), &company.into_inner()) {
        Ok(v) => ApiResponse{ json: json!(v), status: Status::Ok},
        Err(err) => ApiResponse{ json: json!(ErrorJson{message: err}), status: Status::InternalServerError}
    }
}

#[delete("/<id>")]
pub fn delete(pool: &State<core::sqlx::MySqlPool>, id :i32) -> ApiResponse {
    match core::company::Company::delete(pool.inner(), &id) {
        Ok(v) => ApiResponse{ json: json!(v), status: Status::Ok},
        Err(err) => ApiResponse{ json: json!(ErrorJson{message: err}), status: Status::InternalServerError}
    }
}
