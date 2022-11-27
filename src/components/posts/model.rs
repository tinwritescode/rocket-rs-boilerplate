use diesel::prelude::*;
use serde::Serialize;

use crate::schema::*;

#[derive(Queryable, Clone, Serialize, PartialEq, AsChangeset, Selectable)]
#[table_name = "posts"]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    // pub created_at: PgTimestamp,
    // pub updated_at: PgTimestamp,
    // pub user_id: i32,
}
