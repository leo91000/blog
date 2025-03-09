use crate::app::CurrentUser;
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
    let post_resource = Resource::new(post_id, get_post);

    // Create resource to fetch current user
    let user_resource = expect_context::<CurrentUser>();

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
        <div class="max-w-3xl mx-auto w-full">
            <Await future=post_resource.into_future() let:_server_post>
                {move || match post_resource.get() {
                    None => view! { <p class="dark:text-gray-300">"Loading..."</p> }.into_any(),
                    Some(Err(e)) => {
                        view! {
                            <p class="text-red-500 dark:text-red-400">
                                "Error loading post: " {e.to_string()}
                            </p>
                        }
                            .into_any()
                    }
                    Some(Ok(post)) => {
                        view! {
                            <article class="bg-white dark:bg-primary-800 p-8 rounded-xl shadow-lg dark:shadow-primary-900/50 border border-gray-100 dark:border-primary-700">
                                <h1 class="text-3xl font-bold mb-2 dark:text-white">
                                    {post.title.clone()}
                                </h1>
                                <div class="text-gray-500 dark:text-gray-300 mb-6">
                                    {format_date(post.created_at)}
                                </div>

                                // Show admin actions if user is admin
                                <Await future=user_resource.into_future() let:_server_user>
                                    {move || {
                                        let user = user_resource.get().flatten();
                                        if user.is_some_and(|user| user.is_admin) {
                                            view! {
                                                <div class="flex gap-4 mb-6">
                                                    <A
                                                        href=format!("/blog/{}/edit", post.id)
                                                        attr:class="px-4 py-2 bg-blue-500 dark:bg-blue-600 text-white rounded-lg hover:bg-blue-600 dark:hover:bg-blue-700 transition-colors"
                                                    >
                                                        <span class="flex items-center gap-2">
                                                            <span class="i-mdi-pencil"></span>
                                                            "Edit"
                                                        </span>
                                                    </A>
                                                    <button
                                                        class="px-4 py-2 bg-red-500 dark:bg-red-600 text-white rounded-lg hover:bg-red-600 dark:hover:bg-red-700 transition-colors flex items-center gap-2"
                                                        on:click=move |_| {
                                                            handle_delete(post.id);
                                                        }
                                                    >
                                                        <span class="i-mdi-delete"></span>
                                                        "Delete"
                                                    </button>
                                                </div>
                                            }
                                                .into_any()
                                        } else {
                                            ().into_any()
                                        }
                                    }}
                                </Await>

                                <div class="prose dark:prose-invert max-w-none">
                                    // Display content - in a real app, you might want to render markdown
                                    {post
                                        .content
                                        .split("\n")
                                        .map(|p| {
                                            view! {
                                                <p class="mb-4 dark:text-gray-200">{p.to_string()}</p>
                                            }
                                        })
                                        .collect::<Vec<_>>()}
                                </div>
                            </article>

                            <div class="mt-8">
                                <A
                                    href="/blog"
                                    attr:class="text-primary-600 dark:text-primary-400 hover:underline flex items-center gap-1"
                                >
                                    <span class="i-mdi-arrow-left"></span>
                                    "Back to all posts"
                                </A>
                            </div>
                        }
                            .into_any()
                    }
                }}
            </Await>
        </div>
    }
}
