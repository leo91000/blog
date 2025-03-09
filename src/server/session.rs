use leptos::prelude::*;

use crate::models::session::ThemePreference;

type Result<T> = std::result::Result<T, ServerFnError>;

#[server(GetThemepPreference, "/api/session")]
pub async fn get_theme_preference() -> Result<ThemePreference> {
    use super::utils::session::{get_user_session, set_user_session};
    use crate::models::session::SessionUser;
    use axum::http::HeaderMap;
    use leptos_axum::extract;

    match get_user_session().await {
        Ok(Some(user)) => Ok(user.theme_preference),
        Ok(None) => {
            let request_header: HeaderMap = extract().await?;

            let theme_preference: ThemePreference = request_header
                .get("Sec-CH-Prefers-Color-Scheme")
                .and_then(|value| value.to_str().ok())
                .and_then(|value| value.parse().ok())
                .unwrap_or(ThemePreference::System);

            set_user_session(&SessionUser {
                id: None,
                username: None,
                is_admin: false,
                theme_preference,
            })
            .await?;
            Ok(ThemePreference::System)
        }
        _ => Err(ServerFnError::new("Not found")),
    }
}

#[server(SetThemePreference, "/api/session")]
pub async fn set_theme_preference(theme_preference: ThemePreference) -> Result<()> {
    use super::utils::{
        db::get_db,
        session::{get_user_session, set_user_session},
    };
    use crate::models::session::SessionUser;

    match get_user_session().await {
        Ok(Some(mut user)) => {
            if user.theme_preference == theme_preference {
                return Ok(());
            }
            user.theme_preference = theme_preference;
            set_user_session(&user).await?;
            if let Some(user_id) = user.id {
                let conn = get_db();
                conn.execute(
                    "UPDATE users SET theme_preference = ? WHERE id = ?",
                    libsql::params![theme_preference, user_id],
                )
                .await?;
            }
            Ok(())
        }
        Ok(None) => {
            set_user_session(&SessionUser {
                id: None,
                username: None,
                is_admin: false,
                theme_preference,
            })
            .await?;
            Ok(())
        }
        _ => Err(ServerFnError::new("Not found")),
    }
}
