use rocket::form::Form;
use rocket::serde::json::Json;

use crate::components::posts::service::fetch_posts;
use crate::{base::model::BaseResponse, components::Post};

#[derive(FromForm)]
pub struct LimitParam {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

pub fn routes() -> Vec<rocket::Route> {
    routes![index]
}

#[get("/", format = "json", data = "<limit_param>")]
fn index(limit_param: Form<LimitParam>) -> BaseResponse<Vec<Post>> {
    let conn = &mut crate::establish_connection();
    let limit = limit_param.limit.unwrap_or(10);
    let offset = limit_param.offset.unwrap_or(0);

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
