use okapi::openapi3::OpenApi;
use rocket::serde::json::Json;
use rocket_okapi::{openapi, openapi_get_routes_spec};

use crate::components::posts::service::fetch_posts;
use crate::PostJson;
use crate::{base::BaseResponse, components::Post};

#[derive(FromForm)]
pub struct LimitParam {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

pub fn routes() -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![index]
}

#[openapi]
#[get("/?<offset>&<limit>")]
fn index(offset: i64, limit: i64) -> BaseResponse<Vec<PostJson>> {
    let conn = &mut crate::establish_connection();

    let posts = match fetch_posts(
        LimitParam {
            limit: Some(limit),
            offset: Some(offset),
        },
        conn,
    ) {
        Ok(posts) => posts,
        Err(err) => {
            return Err(err);
        }
    };

    Ok(Json(posts.to_vec()))
}
