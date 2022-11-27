use std::time::SystemTime;

use chrono::{DateTime, Utc};
use diesel::prelude::*;
use schemars::JsonSchema;
use serde::Serialize;

use crate::schema::*;

#[derive(Queryable, Serialize, AsChangeset, Selectable, Identifiable, Debug)]
#[diesel(table_name = posts)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
}

impl Post {
    pub fn attach(self) -> PostJson {
        PostJson {
            body: self.body,
            title: self.title,
            created_at: DateTime::<Utc>::from(self.created_at).to_string(),
            updated_at: DateTime::<Utc>::from(self.updated_at).to_string(),
        }
    }
}

#[derive(Serialize, JsonSchema, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PostJson {
    pub title: String,
    pub body: String,
    pub created_at: String,
    pub updated_at: String,
}
