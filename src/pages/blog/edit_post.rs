use crate::models::post::{Post, UpdatePostData};
use crate::server::auth::get_current_user;
use crate::server::blog::{get_post, update_post};
use leptos::{ev, prelude::*, task::spawn_local};
use leptos_router::{components::A, hooks::use_params_map};

/// Edit post page
#[component]
pub fn EditPostPage() -> impl IntoView {
    let params = use_params_map();
    let post_id = move || {
        params
            .get()
            .get("id")
            .and_then(|id| id.parse::<i64>().ok())
            .unwrap_or(0)
    };

    // Check if user is admin
    let user_resource =
        OnceResource::new(async move { (get_current_user().await).unwrap_or_default() });

    // Create resource to fetch post
    let post_resource = OnceResource::new(async move { get_post(post_id()).await });

    // Form state
    let (title, set_title) = signal(String::new());
    let (content, set_content) = signal(String::new());
    let (published, set_published) = signal(false);
    let (error, set_error) = signal(Option::<String>::None);

    // Initialize form with post data when loaded
    Effect::new(move |_| {
        if let Some(Ok(post)) = post_resource.get() {
            set_title.set(post.title);
            set_content.set(post.content);
            set_published.set(post.published);
        }
    });

    // Update result storage
    let (update_result, set_update_result) = signal(None::<Result<Post, ServerFnError>>);
    let navigate = leptos_router::hooks::use_navigate();

    let on_submit = move |ev: ev::SubmitEvent| {
        ev.prevent_default();

        // Validate inputs
        if title.get().trim().is_empty() || content.get().trim().is_empty() {
            set_error.set(Some("Title and content are required".to_string()));
            return;
        }

        // Create update data
        let update_data = UpdatePostData {
            id: post_id(),
            title: title.get(),
            content: content.get(),
            published: published.get(),
        };

        spawn_local(async move {
            let result = update_post(update_data).await;
            set_update_result.set(Some(result));
        });
    };

    // Handle action result
    Effect::new(move |_| {
        if let Some(result) = update_result.get() {
            match result {
                Ok(post) => {
                    // Redirect to the updated post
                    navigate(&format!("/blog/{}", post.id), Default::default());
                }
                Err(e) => {
                    // Show error message
                    set_error.set(Some(e.to_string()));
                }
            }
        }
    });

    view! {
        <div class="max-w-3xl mx-auto">
            // Check if user is admin
            {move || {
                let user = user_resource.get().flatten();
                match user {
                    Some(user) if user.is_admin => {
                        view! {
                            {move || match post_resource.get() {
                                None => view! { <p>"Loading..."</p> }.into_any(),
                                Some(Err(e)) => {
                                    view! {
                                        <p class="text-red-500">
                                            "Error loading post: " {e.to_string()}
                                        </p>
                                    }
                                        .into_any()
                                }
                                Some(Ok(_)) => {
                                    view! {
                                        <div>
                                            <h1 class="text-3xl font-bold mb-6">"Edit Post"</h1>

                                            {move || {
                                                error
                                                    .get()
                                                    .map(|err| {
                                                        view! {
                                                            <div
                                                                class="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded mb-4"
                                                                role="alert"
                                                            >
                                                                <span class="block sm:inline">{err}</span>
                                                            </div>
                                                        }
                                                    })
                                            }}

                                            <form
                                                on:submit=on_submit
                                                class="bg-white p-6 rounded shadow"
                                            >
                                                <div class="mb-4">
                                                    <label
                                                        for="title"
                                                        class="block text-gray-700 font-bold mb-2"
                                                    >
                                                        "Title"
                                                    </label>
                                                    <input
                                                        type="text"
                                                        id="title"
                                                        class="w-full px-3 py-2 border border-gray-300 rounded focus:outline-none focus:border-blue-500"
                                                        placeholder="Post title"
                                                        on:input=move |ev| {
                                                            set_title.set(event_target_value(&ev));
                                                        }
                                                        prop:value=title
                                                    />
                                                </div>

                                                <div class="mb-4">
                                                    <label
                                                        for="content"
                                                        class="block text-gray-700 font-bold mb-2"
                                                    >
                                                        "Content"
                                                    </label>
                                                    <textarea
                                                        id="content"
                                                        class="w-full px-3 py-2 border border-gray-300 rounded focus:outline-none focus:border-blue-500"
                                                        rows="12"
                                                        placeholder="Write your post content here..."
                                                        on:input=move |ev| {
                                                            set_content.set(event_target_value(&ev));
                                                        }
                                                        prop:value=content
                                                    ></textarea>
                                                </div>

                                                <div class="mb-6">
                                                    <label class="inline-flex items-center">
                                                        <input
                                                            type="checkbox"
                                                            class="form-checkbox h-5 w-5 text-blue-600"
                                                            on:input=move |ev| {
                                                                set_published.set(event_target_checked(&ev));
                                                            }
                                                            prop:checked=published
                                                        />
                                                        <span class="ml-2 text-gray-700">"Published"</span>
                                                    </label>
                                                </div>

                                                <div class="flex justify-between">
                                                    <button
                                                        type="submit"
                                                        class="bg-blue-500 text-white py-2 px-4 rounded hover:bg-blue-600 focus:outline-none focus:bg-blue-600"
                                                    >
                                                        "Update Post"
                                                    </button>

                                                    <A
                                                        href=format!("/blog/{}", post_id())
                                                        attr:class="py-2 px-4 border border-gray-300 rounded hover:bg-gray-100"
                                                    >
                                                        "Cancel"
                                                    </A>
                                                </div>
                                            </form>
                                        </div>
                                    }
                                        .into_any()
                                }
                            }}
                        }
                            .into_any()
                    }
                    _ => {
                        view! {
                            <div class="text-center py-12">
                                <h1 class="text-2xl font-bold mb-4">"Access Denied"</h1>
                                <p class="mb-6">"You must be an admin to edit posts."</p>
                                <A href="/login" attr:class="text-blue-600 hover:underline">
                                    "Login"
                                </A>
                            </div>
                        }
                            .into_any()
                    }
                }
            }}
        </div>
    }
}

