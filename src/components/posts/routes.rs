use okapi::openapi3::OpenApi;

use rocket::http::Status;
use rocket::serde::json::Json;
use rocket_okapi::{openapi, openapi_get_routes_spec};
use schemars::_serde_json;
use validator::Validate;

use crate::base::BaseResponse;
use crate::components::posts::service::fetch_posts;
use crate::require_auth::RequireAccessToken;
use crate::{
    create_post, fetch_post_by_slug, remove_post_by_id, update_post_by_id, DataResult,
    ErrorResponse, NewPostInput, PostJson, UpdatePostInput,
};

#[derive(FromForm)]
pub struct LimitParam {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

pub fn routes() -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![
        index,
        get_single_post,
        post_create_post,
        delete_post,
        update_post
    ]
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

#[openapi(tag = "Blog")]
#[get("/<slug>")]
fn get_single_post(slug: &str) -> BaseResponse<PostJson> {
    let conn = &mut crate::establish_connection();

    match fetch_post_by_slug(slug, conn) {
        Ok(post) => Ok(Json(post)),
        Err(err) => Err(err),
    }
}

#[openapi(tag = "Blog")]
#[post("/", data = "<post>")]
fn post_create_post(
    access_token: RequireAccessToken,
    post: DataResult<'_, NewPostInput>,
) -> BaseResponse<PostJson> {
    println!("{:?}", access_token);
    let conn = &mut crate::establish_connection();

    let post = post.expect("Failed to parse post");
    if let Err(e) = post.validate() {
        return Err((
            Status::BadRequest,
            Json(ErrorResponse::new_with_data(
                e.to_string(),
                Some(
                    e.into_errors()
                        .into_iter()
                        .map(|e| _serde_json::json!(e))
                        .collect(),
                ),
            )),
        ));
    }

    match create_post(post.0, conn) {
        Ok(post) => Ok(Json(post)),
        Err(err) => Err(err),
    }
}

#[openapi(tag = "Blog")]
#[delete("/<id>")]
fn delete_post(_access_token: RequireAccessToken, id: i32) -> BaseResponse<PostJson> {
    let conn = &mut crate::establish_connection();

    // TODO: check if user is owner of post or admin

    match remove_post_by_id(id, conn) {
        Ok(post) => Ok(Json(post)),
        Err(err) => Err(err),
    }
}

#[openapi(tag = "Blog")]
#[put("/<id>", data = "<post>")]
fn update_post(id: i32, post: DataResult<'_, UpdatePostInput>) -> BaseResponse<PostJson> {
    let conn = &mut crate::establish_connection();

    // TODO: check if user is owner of post or admin

    let post = post.expect("Failed to parse post");
    if let Err(e) = post.validate() {
        return Err((
            Status::BadRequest,
            Json(ErrorResponse::new_with_data(
                e.to_string(),
                Some(
                    e.into_errors()
                        .into_iter()
                        .map(|e| _serde_json::json!(e))
                        .collect(),
                ),
            )),
        ));
    }

    match update_post_by_id(id, post.0, conn) {
        Ok(post) => Ok(Json(post)),
        Err(err) => Err(err),
    }
}
