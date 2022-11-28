use std::time::SystemTime;

use chrono::{DateTime, Utc};
use diesel::prelude::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::schema::*;

#[derive(Queryable, Serialize, AsChangeset, Selectable, Identifiable, Debug)]
#[diesel(table_name = posts)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub slug: String,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
}

impl Post {
    pub fn attach(self) -> PostJson {
        PostJson {
            id: self.id,
            body: self.body,
            title: self.title,
            slug: self.slug,
            created_at: DateTime::<Utc>::from(self.created_at).to_string(),
            updated_at: DateTime::<Utc>::from(self.updated_at).to_string(),
        }
    }
}

#[derive(Serialize, JsonSchema, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PostJson {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub slug: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = posts)]
pub struct NewPost {
    pub title: String,
    pub body: String,
    pub slug: String,
}

#[derive(AsChangeset)]
#[diesel(table_name = posts)]
pub struct UpdatePost {
    pub title: Option<String>,
    pub body: Option<String>,
}

#[derive(Deserialize, Validate, JsonSchema, FromForm)]
pub struct NewPostInput {
    #[validate(length(
        min = 1,
        max = 255,
        message = "Title must be between 1 and 255 characters"
    ))]
    pub title: String,
    #[validate(length(
        min = 1,
        max = 2000,
        message = "Body must be between 1 and 2000 characters"
    ))]
    pub body: String,
}

#[derive(Deserialize, Validate, JsonSchema, FromForm)]
pub struct UpdatePostInput {
    #[validate(length(
        min = 1,
        max = 255,
        message = "Title must be between 1 and 255 characters"
    ))]
    pub title: Option<String>,
    #[validate(length(
        min = 1,
        max = 2000,
        message = "Body must be between 1 and 2000 characters"
    ))]
    pub body: Option<String>,
    #[validate(length(
        min = 1,
        max = 255,
        message = "Slug must be between 1 and 255 characters"
    ))]
    pub slug: Option<String>,
}
