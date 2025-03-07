use axum::http::header::SET_COOKIE;
use axum::http::{HeaderName, HeaderValue};
use leptos::prelude::*;
use leptos_axum::ResponseOptions;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{LazyLock, RwLock};
use std::time::{Duration, Instant};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SessionUser {
    pub id: i64,
    pub username: String,
    pub is_admin: bool,
}

// For simplicity, we're using a server-side memory store
// In production, you'd want to use Redis, database, or another persistent store
struct SessionData {
    user: Option<SessionUser>,
    expires: Instant,
}

static SESSIONS: LazyLock<RwLock<HashMap<String, SessionData>>> =
    LazyLock::new(|| RwLock::new(HashMap::new()));

async fn get_session_id() -> Result<String, ServerFnError> {
    let header_map: axum::http::HeaderMap = leptos_axum::extract().await?;

    header_map
        .get("cookie")
        .and_then(|cookies| {
            cookies.to_str().ok()?.split(';').find_map(|cookie| {
                let cookie = cookie.trim();
                cookie
                    .strip_prefix("session=")
                    .map(|cookie| cookie.to_string())
            })
        })
        .ok_or_else(|| ServerFnError::new("No session cookie found"))
}

pub async fn get_user() -> Result<Option<SessionUser>, ServerFnError> {
    let Ok(session_id) = get_session_id().await else {
        return Ok(None);
    };

    if let Some(session) = SESSIONS.read()?.get(&session_id) {
        if session.expires > Instant::now() {
            return Ok(session.user.clone());
        }
    }

    Ok(None)
}

pub async fn set_user(user: SessionUser) -> Result<(), ServerFnError> {
    let session_id = uuid::Uuid::new_v4().to_string();
    let session_data = SessionData {
        user: Some(user),
        expires: Instant::now() + Duration::from_secs(24 * 60 * 60), // 24 hours
    };

    SESSIONS.write()?.insert(session_id.clone(), session_data);

    let header_name: HeaderName = "Set-Cookie".parse().unwrap();
    let header_value: HeaderValue = HeaderValue::from_str(&format!(
        "session={}; Path=/; HttpOnly; SameSite=Strict; Max-Age=86400",
        session_id
    ))?;

    let response = expect_context::<ResponseOptions>();
    response.insert_header(header_name, header_value);

    Ok(())
}

pub async fn clear_user() -> Result<(), ServerFnError> {
    if let Ok(session_id) = get_session_id().await {
        SESSIONS.write()?.remove(&session_id);
    }

    let header_name = SET_COOKIE;
    let header_value: HeaderValue =
        "session=; Path=/; HttpOnly; SameSite=Strict; Expires=Thu, 01 Jan 1970 00:00:00 GMT"
            .parse()
            .unwrap();

    let response = expect_context::<ResponseOptions>();
    response.insert_header(header_name, header_value);

    Ok(())
}
