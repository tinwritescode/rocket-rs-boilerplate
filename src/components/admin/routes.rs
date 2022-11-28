use okapi::openapi3::OpenApi;
use rocket::serde::json::Json;
use rocket_okapi::{openapi, openapi_get_routes_spec};

use crate::{base::BaseResponse, components::require_auth::RequireAccessToken};

#[openapi(tag = "Admin")]
#[get("/")]
fn index(_a: RequireAccessToken) -> BaseResponse<&'static str> {
    println!("{:?}", _a);

    Ok(Json("Hello, admin!"))
}

pub fn routes() -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![index]
}
