use validator::ValidationError;

pub fn is_email(email: &str) -> Result<(), ValidationError> {
    let email_regex =
        regex::Regex::new(r"^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\.[a-zA-Z0-9-.]+$").unwrap();

    if let true = email_regex.is_match(email) {
        Ok(())
    } else {
        return Err(ValidationError::new("email"));
    }
}
