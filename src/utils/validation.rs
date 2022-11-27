use rocket::form;

pub fn is_email<'v>(email: &'v str) -> form::Result<'v, ()> {
    let email_regex =
        regex::Regex::new(r"^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\.[a-zA-Z0-9-.]+$").unwrap();

    match email_regex.is_match(email) {
        true => Ok(()),
        false => Err(form::Error::validation("Invalid email").into()),
    }
}
