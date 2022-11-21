use diesel::{ExpressionMethods, SqliteConnection};
use diesel::{QueryDsl, RunQueryDsl};
use rocket::http::Status;
use rocket::serde::json::Json;

use crate::error_handler::ErrorResponse;
use crate::services::bcrypt::hash;

use super::{NewUser, UserWithPassword};

pub fn create_user(
    &user: &NewUser,
    conn: &mut SqliteConnection,
) -> Result<usize, (Status, Json<ErrorResponse>)> {
    use crate::schema::users::dsl::*;

    let mut user = user.clone();
    let binding = hash(&user.password).map_err(|err| {
        (
            Status::InternalServerError,
            Json(ErrorResponse::new(err.to_string())),
        )
    })?;

    user.password = &binding;

    let result = diesel::insert_into(users).values(user).execute(conn);

    match result {
        Ok(insert) => Ok(insert),
        Err(err) => Err((
            Status::InternalServerError,
            Json(ErrorResponse::new(err.to_string())),
        )),
    }
}

pub fn fetch_user_by_email(
    email_address: &str,
    conn: &mut SqliteConnection,
) -> Result<UserWithPassword, (Status, Json<ErrorResponse>)> {
    use crate::schema::users::dsl::*;

    let user = users
        .select(((id, name, email), password))
        .filter(email.eq(email_address))
        .first::<UserWithPassword>(conn);

    match user {
        Ok(user) => Ok(user),
        Err(err) => Err((
            Status::InternalServerError,
            Json(ErrorResponse::new(err.to_string())),
        )),
    }
}
