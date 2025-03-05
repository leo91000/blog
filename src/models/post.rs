use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Post {
    pub id: i64,
    pub title: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub published: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewPost {
    pub title: String,
    pub content: String,
    pub published: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePostData {
    pub id: i64,
    pub title: String,
    pub content: String,
    pub published: bool,
}
