use crate::server::auth::get_current_user;
use crate::server::blog::{delete_post, get_post};
use leptos::{prelude::*, task::spawn_local};
use leptos_router::{components::A, hooks::use_params_map};

/// Single post page
#[component]
pub fn PostPage() -> impl IntoView {
    let params = use_params_map();
    let post_id = move || {
        params
            .get()
            .get("id")
            .and_then(|id| id.parse::<i64>().ok())
            .unwrap_or(0)
    };

    // Create resource to fetch post
    let post_resource = OnceResource::new(async move { get_post(post_id()).await });

    // Create resource to fetch current user
    let user_resource =
        OnceResource::new(async move { (get_current_user().await).unwrap_or_default() });

    // Delete result storage
    let (delete_result, set_delete_result) = signal(None::<Result<(), ServerFnError>>);
    let navigate = leptos_router::hooks::use_navigate();

    // Handle delete action
    let handle_delete = move |id: i64| {
        spawn_local(async move {
            let result = delete_post(id).await;
            set_delete_result.set(Some(result));
        });
    };

    // Handle delete action result
    Effect::new(move |_| {
        if let Some(Ok(_)) = delete_result.get() {
            navigate("/blog", Default::default());
        }
    });

    // Format date for display
    let format_date = |date: chrono::DateTime<chrono::Utc>| date.format("%B %d, %Y").to_string();

    view! {
        <div class="max-w-3xl mx-auto">
            {move || match post_resource.get() {
                None => view! { <p>"Loading..."</p> }.into_any(),
                Some(Err(e)) => {
                    view! { <p class="text-red-500">"Error loading post: " {e.to_string()}</p> }
                        .into_any()
                }
                Some(Ok(post)) => {
                    view! {
                        <article class="bg-white p-8 rounded shadow">
                            <h1 class="text-3xl font-bold mb-2">{post.title.clone()}</h1>
                            <div class="text-gray-500 mb-6">{format_date(post.created_at)}</div>

                            // Show admin actions if user is admin
                            {move || {
                                let user = user_resource.get().flatten();
                                if let Some(user) = user {
                                    if user.is_admin {
                                        view! {
                                            <div class="flex gap-4 mb-6">
                                                <A
                                                    href=format!("/blog/{}/edit", post.id)
                                                    attr:class="px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600"
                                                >
                                                    "Edit"
                                                </A>
                                                <button
                                                    class="px-4 py-2 bg-red-500 text-white rounded hover:bg-red-600"
                                                    on:click=move |_| {
                                                        handle_delete(post.id);
                                                    }
                                                >
                                                    "Delete"
                                                </button>
                                            </div>
                                        }
                                            .into_any()
                                    } else {
                                        ().into_any()
                                    }
                                } else {
                                    ().into_any()
                                }
                            }}

                            <div class="prose max-w-none">
                                // Display content - in a real app, you might want to render markdown
                                {post
                                    .content
                                    .split("\n")
                                    .map(|p| view! { <p class="mb-4">{p.to_string()}</p> })
                                    .collect::<Vec<_>>()}
                            </div>
                        </article>

                        <div class="mt-8">
                            <A href="/blog" attr:class="text-blue-600 hover:underline">
                                "← Back to all posts"
                            </A>
                        </div>
                    }
                        .into_any()
                }
            }}
        </div>
    }
}

