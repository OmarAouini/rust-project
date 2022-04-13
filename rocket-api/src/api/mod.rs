use rocket::http::Status;

pub mod company;

///health check api handler
#[get("/health")]
pub fn health() -> Status {
    Status::Ok
}