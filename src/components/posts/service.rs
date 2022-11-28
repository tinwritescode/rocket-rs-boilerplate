use super::{LimitParam, Post};
use crate::{
    base::BaseServiceResult, slug::generate_slug, ErrorResponse, NewPost, NewPostInput, PostJson,
    UpdatePost, UpdatePostInput,
};
use diesel::{ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl, SelectableHelper};

use rocket::{http::Status, serde::json::Json};

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

pub fn fetch_post_by_id(post_id: i32, conn: &mut PgConnection) -> BaseServiceResult<PostJson> {
    use crate::schema::posts::dsl::*;

    let result = posts
        .find(post_id)
        .select(Post::as_select())
        .first::<Post>(conn);

    match result {
        Ok(post) => Ok(post.attach()),
        Err(err) => Err((Status::NotFound, Json(ErrorResponse::new(err.to_string())))),
    }
}

pub fn fetch_post_by_slug(post_slug: &str, conn: &mut PgConnection) -> BaseServiceResult<PostJson> {
    use crate::schema::posts::dsl::*;

    let result = posts
        .filter(slug.eq(post_slug))
        .select(Post::as_select())
        .first::<Post>(conn);

    match result {
        Ok(post) => Ok(post.attach()),
        Err(err) => Err((Status::NotFound, Json(ErrorResponse::new(err.to_string())))),
    }
}

pub fn create_post(
    post_data: NewPostInput,
    conn: &mut PgConnection,
) -> BaseServiceResult<PostJson> {
    use crate::schema::posts::dsl::*;

    let post_slug = generate_slug(&post_data.title);

    let result = diesel::insert_into(posts)
        .values(NewPost {
            title: post_data.title,
            slug: post_slug,
            body: post_data.body,
        })
        .returning(Post::as_select())
        .get_result::<Post>(conn);

    match result {
        Ok(post) => Ok(post.attach()),
        Err(err) => Err((
            Status::InternalServerError,
            Json(ErrorResponse::new(err.to_string())),
        )),
    }
}

pub fn remove_post_by_id(post_id: i32, conn: &mut PgConnection) -> BaseServiceResult<PostJson> {
    use crate::schema::posts::dsl::*;

    let result = diesel::delete(posts.find(post_id))
        .returning(Post::as_select())
        .get_result::<Post>(conn);

    match result {
        Ok(post) => Ok(post.attach()),
        Err(err) => Err((
            Status::InternalServerError,
            Json(ErrorResponse::new(err.to_string())),
        )),
    }
}

pub fn update_post_by_id(
    post_id: i32,
    post_data: UpdatePostInput,
    conn: &mut PgConnection,
) -> BaseServiceResult<PostJson> {
    use crate::schema::posts::dsl::*;

    let result = diesel::update(posts.find(post_id))
        .set(UpdatePost {
            title: post_data.title,
            body: post_data.body,
        })
        .returning(Post::as_select())
        .get_result::<Post>(conn);

    match result {
        Ok(post) => Ok(post.attach()),
        Err(err) => Err((
            Status::InternalServerError,
            Json(ErrorResponse::new(err.to_string())),
        )),
    }
}

#[cfg(test)]
mod tests {
    use crate::{create_post, NewPostInput};

    #[test]
    fn seed_data() {
        // create post
        let result = create_post(
            NewPostInput {
                title: "Test Post".to_string(),
                body: "This is a test post".to_string(),
            },
            &mut crate::establish_connection(),
        );

        match result {
            Ok(post) => println!("{:#?}", post),
            Err(err) => panic!("{:#?}", err),
        }
    }
}
