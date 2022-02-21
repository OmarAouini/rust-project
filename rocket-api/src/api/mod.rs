use rocket::http::Status;

pub mod company;

#[get("/health")]
pub fn health() -> Status {
    Status::Ok
}