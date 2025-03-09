use chrono::prelude::*;
use serde::{Deserialize, Serialize};

use super::session::{SessionUser, ThemePreference};

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, PartialOrd, Ord)]
pub struct User {
    pub id: i64,
    pub username: String,
    #[cfg(feature = "ssr")]
    #[cfg_attr(feature = "ssr", serde(skip_deserializing))]
    pub password_hash: String,
    pub is_admin: bool,
    pub created_at: DateTime<Utc>,
    pub theme_preference: ThemePreference,
}

impl User {
    pub fn get_session_user(&self) -> SessionUser {
        SessionUser {
            id: Some(self.id),
            username: Some(self.username.clone()),
            is_admin: self.is_admin,
            theme_preference: self.theme_preference,
        }
    }
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
