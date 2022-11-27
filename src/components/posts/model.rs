use std::time::SystemTime;

use diesel::prelude::*;
use schemars::JsonSchema;
use serde::Serialize;

use crate::schema::*;

#[derive(Queryable, Clone, Serialize, AsChangeset, Selectable, Identifiable, JsonSchema)]
#[diesel(table_name = posts)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
}
