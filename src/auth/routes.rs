use super::{create_user, fetch_user_by_email};

use crate::{
    auth::{model::NewUser, User},
    error_handler::ErrorResponse,
};
use rocket::{form::Form, http::Status, serde::json::Json};

pub fn routes() -> Vec<rocket::Route> {
    routes![register, login]
}

#[post("/register", data = "<user>")]
fn register(user: Form<NewUser>) -> Result<Json<User>, (Status, Json<ErrorResponse>)> {
    let user = user.into_inner();

    let conn = &mut crate::establish_connection();

    let insert = match create_user(&user, conn) {
        Ok(insert) => insert,
        Err(err) => {
            return Err(err);
        }
    };

    if insert != 1 {
        return Err((
            Status::InternalServerError,
            Json(ErrorResponse::new("Failed to create user".to_string())),
        ));
    }

    Ok(Json(fetch_user_by_email(&user.email, conn).unwrap().user))
}

#[post("/login")]
fn login() -> &'static str {
    "Hello, world!"
}
