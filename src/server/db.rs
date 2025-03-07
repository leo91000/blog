use libsql::{Builder, Connection, Database};
use std::env;
use std::sync::OnceLock;

type Result<T> = std::result::Result<T, libsql::Error>;

static DB_INSTANCE: OnceLock<Database> = OnceLock::new();

pub async fn init_db() -> Result<()> {
    // Check if we should use Turso or local SQLite
    let use_turso = env::var("USE_TURSO").unwrap_or_else(|_| "false".to_string()) == "true";

    let db = if use_turso {
        let url = env::var("TURSO_DATABASE_URL").expect("TURSO_DATABASE_URL must be set");
        let token = env::var("TURSO_AUTH_TOKEN").expect("TURSO_AUTH_TOKEN must be set");

        Builder::new_remote(url, token).build().await?
    } else {
        Builder::new_local("blog.db").build().await?
    };

    // Initialize database schema
    let conn = db.connect()?;

    // Create users table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY,
            username TEXT NOT NULL UNIQUE,
            password_hash TEXT NOT NULL,
            is_admin BOOLEAN NOT NULL DEFAULT FALSE,
            created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
        )",
        (),
    )
    .await?;

    // Create posts table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS posts (
            id INTEGER PRIMARY KEY,
            title TEXT NOT NULL,
            content TEXT NOT NULL,
            created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
            published BOOLEAN NOT NULL DEFAULT FALSE
        )",
        (),
    )
    .await?;

    DB_INSTANCE.set(db).expect("Database already initialized");
    Ok(())
}

pub fn get_db() -> &'static Database {
    DB_INSTANCE.get().expect("Database not initialized")
}

pub async fn get_connection() -> Result<Connection> {
    get_db().connect()
}
