use std::i64;

use super::{NewUser, TokenType, UserWithPassword};
use crate::base::model::BaseServiceResult;
use crate::components::auth::{Claims, Token, User};
use crate::error_handler::ErrorResponse;
use crate::services::bcrypt::{hash, verify};
use diesel::data_types::PgTimestamp;
use diesel::{ExpressionMethods, PgConnection, SelectableHelper};
use diesel::{QueryDsl, RunQueryDsl};
use rocket::http::Status;
use rocket::serde::json::Json;

pub fn create_user(&user: &NewUser, conn: &mut PgConnection) -> BaseServiceResult<usize> {
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
    conn: &mut PgConnection,
) -> BaseServiceResult<UserWithPassword> {
    use crate::schema::users::dsl::*;

    let user = users
        .select((User::as_select(), password))
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

pub fn fetch_user_by_email_and_password(
    email_address: &str,
    password: &str,
    conn: &mut PgConnection,
) -> BaseServiceResult<UserWithPassword> {
    let user = fetch_user_by_email(email_address, conn);

    match user {
        Ok(user) => {
            if verify(password, &user.password).expect("Password is invalid") {
                Ok(user)
            } else {
                Err((
                    Status::Unauthorized,
                    Json(ErrorResponse::new("Invalid credentials".to_string())),
                ))
            }
        }
        Err(err) => Err(err),
    }
}

pub fn fetch_user_by_id(id: i32, conn: &mut PgConnection) -> BaseServiceResult<UserWithPassword> {
    use crate::schema::users::dsl;

    let user = dsl::users
        .select((User::as_select(), dsl::password))
        .filter(dsl::id.eq(id))
        .first::<UserWithPassword>(conn);

    match user {
        Ok(user) => Ok(user),
        Err(err) => Err((
            Status::InternalServerError,
            Json(ErrorResponse::new(err.to_string())),
        )),
    }
}

pub fn create_token(
    user_id: i32,
    token_type: TokenType,
    conn: &mut PgConnection,
) -> BaseServiceResult<String> {
    use jsonwebtoken::{encode, EncodingKey, Header};

    let secret = std::env::var("SECRET").expect("SECRET must be set");
    let user = fetch_user_by_id(user_id, conn).unwrap();

    match token_type {
        TokenType::AccessToken => {
            let claims = {
                let expire_in_minute = std::env::var("ACCESS_TOKEN_EXPIRE_MINUTES")
                    .expect("ACCESS_TOKEN_EXPIRE_MINUTES must be set");

                Claims {
                    sub: user_id.to_string(),
                    exp: (chrono::Utc::now()
                        + chrono::Duration::minutes(
                            expire_in_minute.parse().expect("Invalid number"),
                        ))
                    .timestamp()
                    .try_into()
                    .expect("Failed to convert timestamp"),
                    role: user.user.role.unwrap_or("user".to_string()),
                }
            };
            let token = encode(
                &Header::default(),
                &claims,
                &EncodingKey::from_secret(secret.as_ref()),
            )
            .expect("Failed to create token");

            Ok(token)
        }
        TokenType::RefreshToken => {
            let token = encode(
                &Header::default(),
                &user_id,
                &EncodingKey::from_secret(secret.as_ref()),
            )
            .expect("Failed to create token");
            use crate::schema::tokens::dsl;

            let expired_time = {
                let refresh_token_expire_in_days =
                    std::env::var("REFRESH_TOKEN_EXPIRE_DAYS").expect("REFRESH_TOKEN must be set");

                (chrono::Utc::now()
                    + chrono::Duration::days(
                        refresh_token_expire_in_days
                            .parse()
                            .expect("Invalid number"),
                    ))
                .naive_utc()
            };

            let result = diesel::insert_into(dsl::tokens)
                .values((
                    dsl::user_id.eq(user_id),
                    dsl::token.eq(&token),
                    dsl::type_.eq("refresh"),
                    dsl::expired_at.eq(expired_time),
                ))
                .execute(conn);

            if let Err(err) = result {
                return Err((
                    Status::InternalServerError,
                    Json(ErrorResponse::new(err.to_string())),
                ));
            }

            Ok(token)
        }
    }
}

pub fn fetch_token(token: &str, conn: &mut PgConnection) -> BaseServiceResult<Token> {
    use crate::schema::tokens::dsl;

    let token = dsl::tokens
        .filter(dsl::token.eq(token))
        .select(Token::as_select())
        .first::<Token>(conn);

    match token {
        Ok(token) => {
            let token_expired_at = PgTimestamp::from(token.expired_at).0;

            if token_expired_at < chrono::Utc::now().timestamp() {
                return Err((
                    Status::Unauthorized,
                    Json(ErrorResponse::new("Token is expired".to_string())),
                ));
            }

            Ok(token)
        }
        Err(err) => Err((
            Status::InternalServerError,
            Json(ErrorResponse::new(err.to_string())),
        )),
    }
}
