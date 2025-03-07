pub mod auth;
pub mod blog;
pub mod home;

// Re-export all page components for easier imports
pub use auth::*;
pub use blog::*;
pub use home::*;