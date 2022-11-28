use rocket::{
    http::Status,
    serde::{json::Json, Serialize},
};
use schemars::{JsonSchema, _serde_json};

#[derive(Debug, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ErrorResponse {
    error: String,
    error_data: Option<_serde_json::Value>,
}

impl ErrorResponse {
    pub fn new(error: String) -> Self {
        Self {
            error,
            error_data: None,
        }
    }

    pub fn new_with_data(error: String, error_data: Option<_serde_json::Value>) -> Self {
        Self { error, error_data }
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

#[catch(422)]
pub fn unprocessable_entity(req: &rocket::Request) -> (Status, Json<ErrorResponse>) {
    (
        Status::UnprocessableEntity,
        Json(ErrorResponse::new(format!("Unprocessable entity"))),
    )
}

#[catch(default)]
pub fn default() -> (Status, Json<ErrorResponse>) {
    (
        Status::InternalServerError,
        Json(ErrorResponse::new("Please try again later".to_string())),
    )
}
