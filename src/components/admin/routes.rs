use rocket::serde::json::Json;

use crate::{base::model::BaseResponse, components::require_auth::AccessToken};

#[get("/")]
fn index(_a: AccessToken) -> BaseResponse<String> {
    Ok(Json("Hello, admin!".to_string()))
}

pub fn routes() -> Vec<rocket::Route> {
    routes![index]
}
