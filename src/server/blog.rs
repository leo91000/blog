use crate::models::post::{NewPost, Post, UpdatePostData};
use leptos::prelude::*;

type Result<T> = std::result::Result<T, ServerFnError>;

#[server(GetPosts, "/api/blog")]
pub async fn get_posts(only_published: bool) -> Result<Vec<Post>> {
    let conn = crate::server::db::get_connection().await?;

    let query = if only_published {
        "SELECT id, title, content, created_at, updated_at, published FROM posts WHERE published = TRUE ORDER BY created_at DESC"
    } else {
        "SELECT id, title, content, created_at, updated_at, published FROM posts ORDER BY created_at DESC"
    };

    let mut rows = conn.query(query, ()).await?;

    let mut posts = Vec::new();
    while let Some(row) = rows.next().await? {
        posts.push(Post {
            id: row.get(0)?,
            title: row.get(1)?,
            content: row.get(2)?,
            created_at: row.get::<String>(3)?.parse()?,
            updated_at: row.get::<String>(4)?.parse()?,
            published: row.get(5)?,
        });
    }

    Ok(posts)
}

#[server(GetPost, "/api/blog")]
pub async fn get_post(id: i64) -> Result<Post> {
    let conn = crate::server::db::get_connection().await?;

    let mut rows = conn
        .query(
            "SELECT id, title, content, created_at, updated_at, published FROM posts WHERE id = ?",
            libsql::params![id],
        )
        .await?;

    let Some(row) = rows.next().await? else {
        return Err(ServerFnError::new("Not found"));
    };

    let post = Post {
        id: row.get(0)?,
        title: row.get(1)?,
        content: row.get(2)?,
        created_at: row.get::<String>(3)?.parse()?,
        updated_at: row.get::<String>(4)?.parse()?,
        published: row.get(5)?,
    };

    // Check if the post is published or the user is admin
    if !post.published {
        let user = crate::server::session::get_user().await?;
        if user.is_none_or(|u| !u.is_admin) {
            return Err(ServerFnError::new("Not found"));
        }
    }

    Ok(post)
}

#[server(CreatePost, "/api/blog")]
pub async fn create_post(new_post: NewPost) -> Result<Post> {
    // Check if user is admin
    let user = crate::server::session::get_user().await?;
    if user.is_none_or(|u| !u.is_admin) {
        return Err(ServerFnError::new("Forbidden"));
    }

    let conn = crate::server::db::get_connection().await?;

    // Get current timestamp
    let now = chrono::Utc::now();

    // Insert the new post
    let mut rows = conn.query(
        "INSERT INTO posts (title, content, created_at, updated_at, published) VALUES (?, ?, ?, ?, ?) RETURNING id",
        libsql::params![new_post.title.clone(), new_post.content.clone(), now.to_string(), now.to_string(), new_post.published]
    ).await?;

    let Some(row) = rows.next().await? else {
        return Err(ServerFnError::new("Failed to insert post"));
    };

    let id = row.get(0)?;

    // Return the created post
    let post = Post {
        id,
        title: new_post.title,
        content: new_post.content,
        created_at: now,
        updated_at: now,
        published: new_post.published,
    };

    Ok(post)
}

#[server(UpdatePost, "/api/blog")]
pub async fn update_post(update: UpdatePostData) -> Result<Post> {
    // Check if user is admin
    let user = crate::server::session::get_user().await?;
    if user.is_none_or(|u| !u.is_admin) {
        return Err(ServerFnError::new("Forbidden"));
    }

    let conn = crate::server::db::get_connection().await?;

    // Get current timestamp
    let now = chrono::Utc::now();

    // Update the post
    let result = conn
        .execute(
            "UPDATE posts SET title = ?, content = ?, updated_at = ?, published = ? WHERE id = ?",
            libsql::params![
                update.title,
                update.content,
                now.to_string(),
                update.published,
                update.id
            ],
        )
        .await?;

    if result == 0 {
        return Err(ServerFnError::new("Not found"));
    }

    // Get the updated post
    get_post(update.id).await
}

#[server(DeletePost, "/api/blog")]
pub async fn delete_post(id: i64) -> Result<()> {
    // Check if user is admin
    let user = crate::server::session::get_user().await?;
    if user.is_none_or(|u| !u.is_admin) {
        return Err(ServerFnError::new("Forbidden"));
    }

    let conn = crate::server::db::get_connection().await?;

    // Delete the post
    let result = conn
        .execute("DELETE FROM posts WHERE id = ?", libsql::params![id])
        .await?;

    if result == 0 {
        return Err(ServerFnError::new("Not found"));
    }

    Ok(())
}
