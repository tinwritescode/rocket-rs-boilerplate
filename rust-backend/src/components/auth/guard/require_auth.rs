use crate::{decode, Claims};
use okapi::openapi3::{Object, SecurityRequirement, SecurityScheme, SecuritySchemeData};
use rocket::{
    http::Status,
    outcome::Outcome,
    request::{self, FromRequest},
    Request,
};
use rocket_okapi::{
    gen::OpenApiGenerator,
    request::{OpenApiFromRequest, RequestHeaderInput},
};

#[derive(Debug)]
pub struct RequireAccessToken(Claims);

#[derive(Debug)]
pub enum AccessTokenError {
    Missing,
    Invalid,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for RequireAccessToken {
    type Error = AccessTokenError;

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let keys: Vec<_> = req.headers().get("Authorization").collect();
        match keys.len() {
            0 => Outcome::Failure((Status::Unauthorized, AccessTokenError::Missing)),
            1 => {
                let token = keys[0];

                if let Ok(claims) = decode(token) {
                    Outcome::Success(RequireAccessToken(claims))
                } else {
                    Outcome::Failure((Status::Unauthorized, AccessTokenError::Invalid))
                }
            }
            _ => Outcome::Failure((Status::Unauthorized, AccessTokenError::Invalid)),
        }
    }
}

impl<'a> OpenApiFromRequest<'a> for RequireAccessToken {
    fn from_request_input(
        _gen: &mut OpenApiGenerator,
        _name: String,
        _required: bool,
    ) -> rocket_okapi::Result<RequestHeaderInput> {
        // Setup global requirement for Security scheme
        let security_scheme = SecurityScheme {
            description: Some(
                "Requires an Bearer token to access, token is: `mytoken`.".to_owned(),
            ),
            // Setup data requirements.
            // In this case the header `Authorization: mytoken` needs to be set.
            data: SecuritySchemeData::Http {
                scheme: "bearer".to_owned(), // `basic`, `digest`, ...
                // Just gives use a hint to the format used
                bearer_format: Some("bearer".to_owned()),
            },
            extensions: Object::default(),
        };
        // Add the requirement for this route/endpoint
        // This can change between routes.
        let mut security_req = SecurityRequirement::new();
        // Each security requirement needs to be met before access is allowed.
        security_req.insert("HttpAuth".to_owned(), Vec::new());
        // These vvvvvvv-----^^^^^^^^ values need to match exactly!
        Ok(RequestHeaderInput::Security(
            "HttpAuth".to_owned(),
            security_scheme,
            security_req,
        ))
    }
}
