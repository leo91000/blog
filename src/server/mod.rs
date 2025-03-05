pub mod auth;
pub mod blog;
pub mod db;
pub mod session;

#[derive(Debug, thiserror::Error)]
pub enum ServerError {
    #[error("Database error: {0}")]
    Database(#[from] libsql::Error),

    #[error("Authentication error: {0}")]
    Auth(String),

    #[error("Not found")]
    NotFound,

    #[error("Forbidden")]
    Forbidden,

    #[error("Server error: {0}")]
    Other(String),
}
