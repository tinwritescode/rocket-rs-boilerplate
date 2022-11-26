use rocket::serde::json::Json;

use crate::{auth::require_auth::AccessToken, base::model::BaseResponse};

#[get("/")]
fn index(_a: AccessToken) -> BaseResponse<String> {
    Ok(Json("Hello, admin!".to_string()))
}

pub fn routes() -> Vec<rocket::Route> {
    routes![index]
}
