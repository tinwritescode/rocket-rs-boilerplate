use crate::error_handler::ErrorResponse;
use rocket::{http::Status, serde::json::Json};
use schemars::JsonSchema;
use serde::Serialize;

pub type DataResult<'a, T> =
    std::result::Result<rocket::serde::json::Json<T>, rocket::serde::json::Error<'a>>;

pub type BaseResponse<T> = Result<Json<ResponseSuccess<T>>, (Status, Json<ErrorResponse>)>;
pub type BaseServiceResult<T> = Result<T, (Status, Json<ErrorResponse>)>;

#[derive(Debug, Serialize, JsonSchema, PartialEq)]
pub struct ResponseSuccess<T> {
    data: T,
}

impl<T> ResponseSuccess<T> {
    pub fn new(data: T) -> Self {
        Self { data }
    }
}
