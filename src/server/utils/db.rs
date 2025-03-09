use libsql::{Builder, Connection};
use std::env;
use std::sync::OnceLock;

type Result<T> = std::result::Result<T, libsql::Error>;

static DB_INSTANCE: OnceLock<Connection> = OnceLock::new();

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

    // Create sessions table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS sessions (
            id TEXT PRIMARY KEY,
            user_id INTEGER,
            username TEXT,
            is_admin BOOLEAN,
            expires TIMESTAMP NOT NULL,
            FOREIGN KEY (user_id) REFERENCES users(id)
        )",
        (),
    )
    .await?;

    // Add theme_preference column to users table if it doesn't exist
    let mut existing_users_tp = conn
        .query(
            "SELECT 1 FROM pragma_table_info('users') WHERE name='theme_preference';",
            (),
        )
        .await?;
    if existing_users_tp.next().await?.is_none() {
        conn.execute(
            "ALTER TABLE users ADD COLUMN theme_preference TEXT NOT NULL DEFAULT 'system'",
            (),
        )
        .await?;
    }

    // Add theme_preference column to sessions table if it doesn't exist
    let mut existing_session_tp = conn
        .query(
            "SELECT 1 FROM pragma_table_info('sessions') WHERE name='theme_preference';",
            (),
        )
        .await?;
    if existing_session_tp.next().await?.is_none() {
        conn.execute(
            "ALTER TABLE sessions ADD COLUMN theme_preference TEXT NOT NULL DEFAULT 'system'",
            (),
        )
        .await?;
    }

    // Make username in session optional if it's not already
    let mut existing_session_username = conn
        .query(
            "SELECT 1 FROM pragma_table_info('sessions') WHERE name='username';",
            (),
        )
        .await?;
    if existing_session_username.next().await?.is_none() {
        // Make column nullable with a temporary table approach since SQLite doesn't support ALTER COLUMN DROP NOT NULL directly
        conn.execute_transactional_batch(
            "
                BEGIN TRANSACTION;
                CREATE TABLE sessions_new (
                    id TEXT PRIMARY KEY,
                    user_id INTEGER,
                    username TEXT, -- Removed NOT NULL constraint
                    is_admin BOOLEAN,
                    expires TIMESTAMP NOT NULL,
                    theme_preference TEXT NOT NULL DEFAULT 'system',
                    FOREIGN KEY (user_id) REFERENCES users(id)
                );
                INSERT INTO sessions_new SELECT * FROM sessions;
                DROP TABLE sessions;
                ALTER TABLE sessions_new RENAME TO sessions;
                COMMIT;
                ",
        )
        .await?;
    }

    #[allow(unused_must_use)]
    DB_INSTANCE.set(conn);

    Ok(())
}

pub fn get_db() -> &'static Connection {
    DB_INSTANCE.get().expect("Database not initialized")
}
