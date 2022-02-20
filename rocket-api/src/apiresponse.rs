use rocket::http::{ContentType, Status};
use rocket::{Request, Response, response, serde};
use rocket::response::Responder;
use rocket::serde::json::{json, Json, Value};

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