use crate::error_handler::ErrorResponse;
use rocket::{http::Status, serde::json::Json};

pub type BaseResponse<T> = Result<Json<T>, (Status, Json<ErrorResponse>)>;
pub type BaseServiceResult<T> = Result<T, (Status, Json<ErrorResponse>)>;
