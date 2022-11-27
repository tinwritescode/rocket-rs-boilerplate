use okapi::openapi3::OpenApi;
use rocket::serde::json::Json;
use rocket_okapi::{openapi, openapi_get_routes_spec};

use crate::{base::model::BaseResponse, components::require_auth::AccessToken};

#[openapi]
#[get("/")]
fn index(_a: AccessToken) -> BaseResponse<&'static str> {
    Ok(Json("Hello, admin!"))
}

pub fn routes() -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![index]
}
