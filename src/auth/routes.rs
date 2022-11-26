use super::{
    create_token, create_user, fetch_user_by_email, fetch_user_by_email_and_password, LoginUser,
};
use crate::{
    auth::{model::NewUser, User, UserWithTokens},
    base::model::BaseResponse,
    error_handler::ErrorResponse,
};
use rocket::{form::Form, http::Status, serde::json::Json};
use serde::Serialize;

pub fn routes() -> Vec<rocket::Route> {
    routes![register, login, refresh]
}

#[post("/register", data = "<user>")]
fn register(user: Form<NewUser>) -> BaseResponse<User> {
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

#[post("/login", data = "<user>")]
fn login(user: Form<LoginUser>) -> BaseResponse<UserWithTokens> {
    let user = user.into_inner();
    let conn = &mut crate::establish_connection();

    let user = match fetch_user_by_email_and_password(&user.email, &user.password, conn) {
        Ok(user) => user,
        Err(err) => {
            return Err(err);
        }
    };

    let (access_token, refresh_token) = (
        create_token(user.user.id, super::TokenType::AccessToken, conn).map_err(|err| err),
        create_token(user.user.id, super::TokenType::RefreshToken, conn).map_err(|err| err),
    );

    match (access_token, refresh_token) {
        (Ok(access_token), Ok(refresh_token)) => Ok(Json(UserWithTokens {
            user: user.user,
            access_token,
            refresh_token,
        })),
        _ => Err((
            Status::InternalServerError,
            Json(ErrorResponse::new("Failed to create tokens".to_string())),
        )),
    }
}

#[derive(FromForm)]
struct RefreshToken {
    refresh_token: String,
}

#[derive(Serialize)]
struct AccessToken {
    pub access_token: String,
}

#[post("/refresh", data = "<refresh_token>")]
fn refresh(refresh_token: Form<RefreshToken>) -> BaseResponse<AccessToken> {
    let refresh_token = refresh_token.into_inner();
    let conn = &mut crate::establish_connection();
    let token = crate::auth::service::fetch_token(&refresh_token.refresh_token, conn);

    match token {
        Ok(token) => {
            let access_token =
                create_token(token.user_id, super::TokenType::AccessToken, conn).map_err(|err| err);

            match access_token {
                Ok(access_token) => Ok(Json(AccessToken { access_token })),
                Err(err) => Err(err),
            }
        }
        Err(err) => Err(err),
    }
}
