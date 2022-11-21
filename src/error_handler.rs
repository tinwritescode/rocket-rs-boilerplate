use rocket::{
    http::Status,
    serde::{json::Json, Serialize},
};

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    error: String,
}

impl ErrorResponse {
    pub fn new(error: String) -> Self {
        Self { error }
    }
}

#[catch(500)]
pub fn internal_error() -> (Status, Json<ErrorResponse>) {
    (
        Status::InternalServerError,
        Json(ErrorResponse::new("Internal server error".to_string())),
    )
}

#[catch(404)]
pub fn not_found(_req: &rocket::Request) -> (Status, Json<ErrorResponse>) {
    (
        Status::NotFound,
        Json(ErrorResponse::new("Not found".to_string())),
    )
}
