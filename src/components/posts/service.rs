use super::{LimitParam, Post};
use crate::base::model::BaseServiceResult;
use diesel::prelude::*;
use diesel::{PgConnection, QueryDsl, RunQueryDsl};
use rocket::serde::json::Json;

pub fn fetch_posts(param: LimitParam, conn: &mut PgConnection) -> BaseServiceResult<Vec<Post>> {
    use crate::schema::posts::dsl::*;

    let limit = param.limit.unwrap_or(10);
    let offset = param.offset.unwrap_or(0);

    let results = posts
        .order(id.desc())
        .limit(limit)
        .offset(offset)
        .select(Post::as_select())
        .get_results::<Post>(conn)
        .expect("Error loading posts");

    Ok((&Json(results)).to_vec())
}
