use crate::schema::*;
use bcrypt;
use diesel::{Identifiable, Insertable, Queryable};
use serde::Serialize;

#[derive(Queryable, Serialize, Debug, PartialEq, Eq, Identifiable)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
}

#[derive(Queryable)]
pub struct UserWithPassword {
    pub user: User,
    pub password: String,
}

#[derive(FromForm, Insertable, Clone, Copy, Debug)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub name: &'a str,
    pub email: &'a str,
    pub password: &'a str,
}
