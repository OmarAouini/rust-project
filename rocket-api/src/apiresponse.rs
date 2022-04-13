//!
//! This module contains api response wrappers
//! Author: Omar Aouini
//! 20/02/2022
//!

use rocket::http::{ContentType, Status};
use rocket::{Request, Response, response};
use rocket::response::Responder;
use rocket::serde::json::Value;
use rocket::serde::{Serialize, Deserialize};


/// default api response wrapper, need to specify a statuscode and a json::Value compliants data
#[derive(Debug)]
pub struct ApiResponse {
    pub(crate) json: Value,
    pub(crate) status: Status,
}

impl<'r> Responder<'r, 'r> for ApiResponse {
    fn respond_to(self, req: &Request) -> response::Result<'r> {
        Response::build_from(self.json.respond_to(&req).unwrap())
            .status(self.status)
            .header(ContentType::JSON)
            .ok()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorJson {
    pub message: String
}