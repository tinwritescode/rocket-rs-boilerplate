use jsonwebtoken::{DecodingKey, Validation};
use rocket::{
    http::Status,
    outcome::Outcome,
    request::{self, FromRequest},
    Request,
};
use rocket_okapi::request::OpenApiFromRequest;
use schemars::JsonSchema;

use crate::components::Claims;

#[derive(OpenApiFromRequest)]
pub struct AccessToken(String);

#[derive(Debug)]
pub enum AccessTokenError {
    Missing,
    Invalid,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AccessToken {
    type Error = AccessTokenError;

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let keys: Vec<_> = req.headers().get("Authorization").collect();
        match keys.len() {
            0 => Outcome::Failure((Status::Unauthorized, AccessTokenError::Missing)),
            1 => {
                let token = keys[0];
                let secret = std::env::var("SECRET").expect("SECRET must be set");

                if jsonwebtoken::decode::<Claims>(
                    // skip bearer
                    [&token[7..]].concat().as_str(),
                    &DecodingKey::from_secret(secret.as_ref()),
                    &Validation::default(),
                )
                .is_ok()
                {
                    Outcome::Success(AccessToken(token.to_string()))
                } else {
                    Outcome::Failure((Status::Unauthorized, AccessTokenError::Invalid))
                }
            }
            _ => Outcome::Failure((Status::Unauthorized, AccessTokenError::Invalid)),
        }
    }
}
