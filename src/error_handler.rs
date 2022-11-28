use rocket::{
    http::Status,
    serde::{json::Json, Serialize},
};
use schemars::{JsonSchema, _serde_json};

#[derive(Debug, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ResponseError {
    error: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    error_data: Option<_serde_json::Value>,
}

impl ResponseError {
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
pub fn internal_error() -> (Status, Json<ResponseError>) {
    (
        Status::InternalServerError,
        Json(ResponseError::new("Internal server error".to_string())),
    )
}

#[catch(404)]
pub fn not_found(_req: &rocket::Request) -> (Status, Json<ResponseError>) {
    (
        Status::NotFound,
        Json(ResponseError::new("Not found".to_string())),
    )
}

#[catch(422)]
pub fn unprocessable_entity(_req: &rocket::Request) -> (Status, Json<ResponseError>) {
    (
        Status::UnprocessableEntity,
        Json(ResponseError::new("Unprocessable entity".to_string())),
    )
}

#[catch(default)]
pub fn default() -> (Status, Json<ResponseError>) {
    (
        Status::InternalServerError,
        Json(ResponseError::new("Please try again later".to_string())),
    )
}
