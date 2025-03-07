use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, PartialOrd, Ord)]
pub struct User {
    pub id: i64,
    pub username: String,
    #[cfg(feature = "ssr")]
    #[cfg_attr(feature = "ssr", serde(skip_deserializing))]
    pub password_hash: String,
    pub is_admin: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewUser {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginCredentials {
    pub username: String,
    pub password: String,
}
