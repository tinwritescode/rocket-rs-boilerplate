use okapi::openapi3::OpenApi;
use rocket::serde::json::Json;
use rocket_okapi::{openapi, openapi_get_routes_spec};

use crate::base::BaseResponse;
use crate::components::posts::service::fetch_posts;
use crate::PostJson;

#[derive(FromForm)]
pub struct LimitParam {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

pub fn routes() -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![index]
}

#[openapi(tag = "Blog")]
#[get("/?<offset>&<limit>")]
fn index(offset: Option<i64>, limit: Option<i64>) -> BaseResponse<Vec<PostJson>> {
    let conn = &mut crate::establish_connection();

    let posts = match fetch_posts(LimitParam { limit, offset }, conn) {
        Ok(posts) => posts,
        Err(err) => {
            return Err(err);
        }
    };

    Ok(Json(posts.to_vec()))
}
