use jsonwebtoken::{DecodingKey, Validation};

use crate::Claims;

pub fn decode(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let secret = std::env::var("SECRET").expect("SECRET must be set");
    jsonwebtoken::decode::<Claims>(
        // skip bearer
        [&token[7..]].concat().as_str(),
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )
    .map(|data| data.claims)
}
