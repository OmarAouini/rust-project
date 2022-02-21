use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::serde::json::serde_json::json;
use rocket::State;
use crate::ApiResponse;

#[post("/count", data="<company>")]
pub fn add(pool: &State<core::sqlx::MySqlPool>, company: Json<core::company::Company>) -> ApiResponse {
    let mut codod = company.into_inner();
    codod.email = "test".to_string();
    ApiResponse{ json: json!(codod), status: Status::Created}
}

#[get("/count")]
pub fn count(pool: &State<core::sqlx::MySqlPool>) -> ApiResponse {
    use core::traits::Crud;

    let all = core::company::Company::find_all(&pool).unwrap();
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
