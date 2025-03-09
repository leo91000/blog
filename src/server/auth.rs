use leptos::prelude::*;

use crate::models::user::{LoginCredentials, NewUser, User};

type Result<T> = std::result::Result<T, ServerFnError>;

#[server(Register, "/api/auth")]
pub async fn register(new_user: NewUser) -> Result<()> {
    use argon2::{
        password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
        Argon2,
    };
    use leptos::prelude::*;
    use libsql::params;

    use super::utils::db::get_db;

    // Hash the password
    let salt = SaltString::generate(&mut OsRng);
    let argon2_config = Argon2::default();
    let password_hash = argon2_config
        .hash_password(new_user.password.as_bytes(), &salt)
        .map_err(|e| ServerFnError::new(e.to_string()))?
        .to_string();

    let conn = get_db();

    // Insert the new user
    let result = conn
        .execute(
            "INSERT INTO users (username, password_hash, is_admin) VALUES (?, ?, FALSE)",
            params![new_user.username, password_hash],
        )
        .await;

    match result {
        Ok(_) => Ok(()),
        Err(e) => {
            if e.to_string().contains("UNIQUE constraint failed") {
                Err(ServerFnError::new("Username already exists"))
            } else {
                Err(ServerFnError::new("Database error"))
            }
        }
    }
}

#[server(Login, "/api/auth")]
pub async fn login(credentials: LoginCredentials) -> Result<User> {
    use crate::models::user::User;
    use argon2::{
        password_hash::{PasswordHash, PasswordVerifier},
        Argon2,
    };
    use leptos::prelude::*;
    use libsql::params;

    use super::utils::db::get_db;
    use super::utils::session;

    tracing::debug!("login 1");
    let conn = get_db();

    // Fetch the user
    let mut rows = conn.query(
        "SELECT id, username, password_hash, is_admin, theme_preference, created_at FROM users WHERE username = ?",
        params![credentials.username.clone()],
    ).await?;

    let first_row = rows.next().await?;

    tracing::debug!("login 2 {:?}", first_row);

    let Some(row) = first_row else {
        return Err(ServerFnError::new("Invalid username or password"));
    };

    let password_hash: String = row.get(2)?;

    // Verify the password
    let parsed_hash =
        PasswordHash::new(&password_hash).map_err(|e| ServerFnError::new(e.to_string()))?;

    let argon2 = Argon2::default();
    if argon2
        .verify_password(credentials.password.as_bytes(), &parsed_hash)
        .is_err()
    {
        return Err(ServerFnError::new("Invalid username or password"));
    }

    let theme_preference = crate::models::session::ThemePreference::from_libsql_value(row.get(4)?);
    let created_at = row.get::<String>(5)?;
    tracing::debug!("login 3 {:?} {:?}", theme_preference, created_at);

    // Get user data
    let user = User {
        id: row.get(0)?,
        username: row.get(1)?,
        password_hash,
        is_admin: row.get(3)?,
        theme_preference,
        created_at: chrono::NaiveDateTime::parse_from_str(
            created_at.as_str(),
            "%Y-%m-%d %H:%M:%S",
        )?
        .and_utc(),
    };

    tracing::debug!("login 4 {:?}", user);

    // Set the user in session
    session::set_user_session(&user.get_session_user()).await?;

    tracing::debug!("login 5 {:?}", user);

    Ok(user)
}

#[server(Logout, "/api/auth")]
pub async fn logout() -> Result<()> {
    use super::utils::session;
    session::clear_user().await?;
    Ok(())
}

#[server(GetCurrentUser, "/api/auth")]
pub async fn get_current_user() -> Result<Option<User>> {
    use crate::models::user::User;

    use libsql::params;

    use super::utils::db::get_db;
    use super::utils::session;

    let session_user = session::get_user_session().await?;

    if let Some(session_user) = session_user {
        let conn = get_db();

        let mut rows = conn
            .query(
                "SELECT id, username, password_hash, is_admin, theme_preference, created_at FROM users WHERE id = ?",
                params![session_user.id],
            )
            .await?;

        let Some(row) = rows.next().await? else {
            return Ok(None);
        };

        let user = User {
            id: row.get(0)?,
            username: row.get(1)?,
            password_hash: row.get(2)?,
            is_admin: row.get(3)?,
            theme_preference: crate::models::session::ThemePreference::from_libsql_value(
                row.get(4)?,
            ),
            created_at: chrono::NaiveDateTime::parse_from_str(
                &row.get::<String>(5)?,
                "%Y-%m-%d %H:%M:%S",
            )?
            .and_utc(),
        };

        let should_be_session_user = user.get_session_user();
        if should_be_session_user != session_user {
            session::set_user_session(&should_be_session_user).await?;
        }

        Ok(Some(user))
    } else {
        Ok(None)
    }
}
