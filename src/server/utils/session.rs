use super::db;
use crate::models::session::{SessionUser, ThemePreference};
use axum::http::header::SET_COOKIE;
use axum::http::{HeaderName, HeaderValue};
use axum_extra::extract::CookieJar;
use leptos::prelude::*;
use leptos_axum::ResponseOptions;
use libsql::params;
use std::time::{SystemTime, UNIX_EPOCH};

async fn get_session_id() -> Result<String, ServerFnError> {
    let cookie_jar: CookieJar = leptos_axum::extract().await?;

    cookie_jar
        .get("session")
        .map(|cookie| cookie.value().to_string())
        .ok_or_else(|| ServerFnError::new("No session cookie found"))
}

pub async fn get_user_session() -> Result<Option<SessionUser>, ServerFnError> {
    let Ok(session_id) = get_session_id().await else {
        return Ok(None);
    };

    let conn = db::get_db();

    // Get the current timestamp in seconds
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let mut rows = conn
        .query(
            "SELECT user_id, username, is_admin, theme_preference FROM sessions WHERE id = ?1 AND expires > datetime(?2, 'unixepoch')",
            params![session_id, now],
        )
        .await?;

    let row = rows
        .next()
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    if let Some(row) = row {
        let user = SessionUser {
            id: row.get(0)?,
            username: row.get(1)?,
            is_admin: row.get(2)?,
            theme_preference: ThemePreference::from_libsql_value(row.get(3)?),
        };
        Ok(Some(user))
    } else {
        Ok(None)
    }
}

pub async fn set_user_session(user: &SessionUser) -> Result<(), ServerFnError> {
    let existing_session_id = get_session_id().await;
    let session_id = existing_session_id
        .clone()
        .unwrap_or_else(|_| uuid::Uuid::new_v4().to_string());

    // Calculate expiration time (24 hours from now)
    let expires = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
        + 24 * 60 * 60;

    let conn = db::get_db();

    // Insert new session into database
    conn.execute(
        "INSERT OR REPLACE INTO sessions (id, user_id, username, is_admin, expires, theme_preference) VALUES (?1, ?2, ?3, ?4, datetime(?5, 'unixepoch'), ?6)",
        params![
            session_id.clone(),
            user.id,
            user.username.clone(),
            user.is_admin,
            expires,
            user.theme_preference,
        ],
    )
    .await
    .map_err(|e| ServerFnError::new(e.to_string()))?;

    // We need to set the cookie in the response unless the user already has a session cookie
    if existing_session_id.is_err() {
        let header_name: HeaderName = "Set-Cookie".parse().unwrap();
        let header_value: HeaderValue = HeaderValue::from_str(&format!(
            "session={}; Path=/; HttpOnly; SameSite=Strict; Max-Age=86400",
            session_id
        ))?;

        let response = expect_context::<ResponseOptions>();
        response.insert_header(header_name, header_value);
    }

    Ok(())
}

pub async fn clear_user() -> Result<(), ServerFnError> {
    if let Ok(session_id) = get_session_id().await {
        let conn = db::get_db();

        // Delete session from database
        conn.execute("DELETE FROM sessions WHERE id = ?1", params![session_id])
            .await?;
    }

    let header_name = SET_COOKIE;
    let header_value = HeaderValue::from_str(
        "session=; Path=/; HttpOnly; SameSite=Strict; Expires=Thu, 01 Jan 1970 00:00:00 GMT",
    )
    .unwrap();

    let response = expect_context::<ResponseOptions>();
    response.insert_header(header_name, header_value);

    Ok(())
}
