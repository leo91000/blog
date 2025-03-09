use leptos::prelude::*;
use serde::{Deserialize, Serialize};

use crate::models::session::ThemePreference;

type Result<T> = std::result::Result<T, ServerFnError>;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetThemePreferenceResponse {
    pub theme_preference: ThemePreference,
    pub theme_preference_header: Option<ThemePreference>,
}

#[server(GetThemePreference, "/api/session")]
pub async fn get_theme_preference() -> Result<GetThemePreferenceResponse> {
    use super::utils::session::{get_user_session, set_user_session};
    use crate::models::session::SessionUser;
    use axum::http::header::VARY;
    use axum::http::HeaderMap;
    use leptos_axum::extract;
    use leptos_axum::ResponseOptions;

    let response: ResponseOptions = expect_context();

    let get_theme_from_header = async move || -> Result<Option<ThemePreference>> {
        let request_header: HeaderMap = extract().await?;
        let color_scheme_header = request_header.get("Sec-CH-Prefers-Color-Scheme");

        let theme_preference = color_scheme_header
            .and_then(|value| value.to_str().ok())
            .and_then(|value| value.parse().ok());

        if color_scheme_header.is_none() {
            response.insert_header(
                "Accept-CH".parse().unwrap(),
                "Sec-CH-Prefers-Color-Scheme".parse().unwrap(),
            );
            response.insert_header(VARY, "Accept-CH".parse().unwrap());
            response.insert_header(
                "Critical-CH".parse().unwrap(),
                "Sec-CH-Prefers-Color-Scheme".parse().unwrap(),
            );
        }

        Ok(theme_preference)
    };

    match get_user_session().await {
        Ok(Some(user)) => {
            let theme_preference = user.theme_preference;
            let theme_preference_header = get_theme_from_header().await?;
            Ok(GetThemePreferenceResponse {
                theme_preference,
                theme_preference_header,
            })
        }
        Ok(None) => {
            let theme_preference = get_theme_from_header().await?;
            set_user_session(&SessionUser {
                id: None,
                username: None,
                is_admin: false,
                theme_preference: theme_preference.unwrap_or_default(),
            })
            .await?;
            Ok(GetThemePreferenceResponse {
                theme_preference: theme_preference.unwrap_or_default(),
                theme_preference_header: theme_preference,
            })
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
