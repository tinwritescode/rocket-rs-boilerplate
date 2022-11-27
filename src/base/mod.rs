use crate::error_handler::ErrorResponse;
use rocket::{http::Status, serde::json::Json};

pub type DataResult<'a, T> =
    std::result::Result<rocket::serde::json::Json<T>, rocket::serde::json::Error<'a>>;

pub type BaseResponse<T> = Result<Json<T>, (Status, Json<ErrorResponse>)>;
pub type BaseServiceResult<T> = Result<T, (Status, Json<ErrorResponse>)>;
