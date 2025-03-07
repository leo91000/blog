use chrono::NaiveDateTime;
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

    use super::db::get_connection;

    // Hash the password
    let salt = SaltString::generate(&mut OsRng);
    let argon2_config = Argon2::default();
    let password_hash = argon2_config
        .hash_password(new_user.password.as_bytes(), &salt)
        .map_err(|e| ServerFnError::new(e.to_string()))?
        .to_string();

    let conn = get_connection().await?;

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
    use crate::server::session::SessionUser;
    use argon2::{
        password_hash::{PasswordHash, PasswordVerifier},
        Argon2,
    };
    use leptos::prelude::*;
    use libsql::params;

    use super::db::get_connection;
    use super::session;

    let conn = get_connection().await?;

    // Fetch the user
    let mut rows = conn.query(
        "SELECT id, username, password_hash, is_admin, created_at FROM users WHERE username = ?",
        params![credentials.username]
    ).await?;

    let first_row = rows.next().await?;

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

    // Get user data
    let user = User {
        id: row.get(0)?,
        username: row.get(1)?,
        password_hash,
        is_admin: row.get(3)?,
        created_at: NaiveDateTime::parse_from_str(
            row.get::<String>(4)?.as_str(),
            "%Y-%m-%d %H:%M:%S",
        )?
        .and_utc(),
    };

    // Set the user in session
    let session_user = SessionUser {
        id: user.id,
        username: user.username.clone(),
        is_admin: user.is_admin,
    };

    session::set_user(session_user.clone()).await?;
    println!("user: {:?}", user);

    Ok(user)
}

#[server(Logout, "/api/auth")]
pub async fn logout() -> Result<()> {
    use super::session;
    session::clear_user().await?;
    Ok(())
}

#[server(GetCurrentUser, "/api/auth")]
pub async fn get_current_user() -> Result<Option<User>> {
    use crate::models::user::User;

    use libsql::params;

    use super::db::get_connection;
    use super::session;
    let session_user = session::get_user().await?;

    if let Some(session_user) = session_user {
        let conn = get_connection().await?;

        let mut rows = conn
            .query(
                "SELECT id, username, password_hash, is_admin, created_at FROM users WHERE id = ?",
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
            created_at: row.get::<String>(4)?.parse()?,
        };

        Ok(Some(user))
    } else {
        Ok(None)
    }
}
