use std::str::FromStr;

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize, Eq, PartialEq, PartialOrd, Ord, Default)]
pub enum ThemePreference {
    #[default]
    #[serde(rename = "system")]
    System,
    #[serde(rename = "light")]
    Light,
    #[serde(rename = "dark")]
    Dark,
}

impl FromStr for ThemePreference {
    type Err = ();

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "light" => Ok(ThemePreference::Light),
            "dark" => Ok(ThemePreference::Dark),
            "system" => Ok(ThemePreference::System),
            _ => Err(()),
        }
    }
}

impl ThemePreference {
    #[cfg(feature = "ssr")]
    pub fn from_libsql_value(value: libsql::Value) -> Self {
        match value {
            libsql::Value::Text(s) => match s.as_str() {
                "light" => ThemePreference::Light,
                "dark" => ThemePreference::Dark,
                _ => Default::default(),
            },
            _ => Default::default(),
        }
    }
}

#[cfg(feature = "ssr")]
impl TryFrom<libsql::Value> for &ThemePreference {
    type Error = libsql::Error;

    fn try_from(value: libsql::Value) -> Result<Self, Self::Error> {
        match value {
            libsql::Value::Text(s) => match s.as_str() {
                "light" => Ok(&ThemePreference::Light),
                "dark" => Ok(&ThemePreference::Dark),
                "system" => Ok(&ThemePreference::System),
                _ => Err(libsql::Error::InvalidColumnType),
            },
            _ => Err(libsql::Error::InvalidColumnType),
        }
    }
}

#[cfg(feature = "ssr")]
impl From<&ThemePreference> for libsql::Value {
    fn from(val: &ThemePreference) -> Self {
        match val {
            ThemePreference::Light => libsql::Value::Text("light".to_string()),
            ThemePreference::Dark => libsql::Value::Text("dark".to_string()),
            ThemePreference::System => libsql::Value::Text("system".to_string()),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, PartialOrd, Ord)]
pub struct SessionUser {
    pub id: Option<i64>,
    pub username: Option<String>,
    pub is_admin: bool,
    pub theme_preference: ThemePreference,
}
