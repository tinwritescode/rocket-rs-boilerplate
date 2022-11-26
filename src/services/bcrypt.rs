use bcrypt::{self, BcryptError};

pub fn hash(password: &str) -> Result<String, BcryptError> {
    bcrypt::hash(password, bcrypt::DEFAULT_COST)
}

pub fn verify(password: &str, hashed_password: &str) -> Result<bool, BcryptError> {
    bcrypt::verify(password, hashed_password)
}
