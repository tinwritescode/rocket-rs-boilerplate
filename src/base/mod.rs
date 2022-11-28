use crate::error_handler::ResponseError;
use rocket::{http::Status, serde::json::Json};
use schemars::JsonSchema;
use serde::Serialize;

pub type DataResult<'a, T> =
    std::result::Result<rocket::serde::json::Json<T>, rocket::serde::json::Error<'a>>;

pub type BaseResponse<T> = Result<Json<ResponseSuccess<T>>, (Status, Json<ResponseError>)>;
pub type BaseServiceResult<T> = Result<T, (Status, Json<ResponseError>)>;

#[derive(Debug, Serialize, JsonSchema, PartialEq)]
pub struct ResponseSuccess<T> {
    data: T,
    meta: Option<PaginateMeta>,
}

impl<T> ResponseSuccess<T> {
    pub fn new(data: T) -> Self {
        ResponseSuccess { data, meta: None }
    }

    pub fn new_with_meta(data: T, meta: PaginateMeta) -> Self {
        ResponseSuccess {
            data,
            meta: Some(meta),
        }
    }
}

#[derive(Debug, JsonSchema, PartialEq, Serialize)]
pub struct PaginateMeta {
    pub limit: i64,
    pub offset: i64,
    pub total: i64,
}

pub struct PaginateResult<T> {
    pub data: T,
    pub meta: PaginateMeta,
}
