use super::{LimitParam, Post};
use crate::{base::BaseServiceResult, PostJson};
use diesel::{PgConnection, QueryDsl, RunQueryDsl, SelectableHelper};

pub fn fetch_posts(param: LimitParam, conn: &mut PgConnection) -> BaseServiceResult<Vec<PostJson>> {
    use crate::schema::posts::dsl::*;

    let limit = param.limit.unwrap_or(10);
    let offset = param.offset.unwrap_or(0);

    let results: Vec<PostJson> = posts
        .limit(limit)
        .offset(offset)
        .select(Post::as_select())
        .get_results::<Post>(conn)
        .expect("Error loading posts")
        .into_iter()
        .map(|post| post.attach())
        .collect();

    Ok(results)
}
