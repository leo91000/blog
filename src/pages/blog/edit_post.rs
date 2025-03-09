use crate::app::CurrentUser;
use crate::models::post::UpdatePostData;
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

    let user_resource = expect_context::<CurrentUser>();
    let post_resource = Resource::new(post_id, async move |post_id| get_post(post_id).await);

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

    let navigate = leptos_router::hooks::use_navigate();

    view! {
        <div class="max-w-3xl mx-auto w-full">
            // Check if user is admin
            <Suspense fallback=|| {
                view! { <div class="dark:text-gray-300">"Loading..."</div> }
            }>
                {move || {
                    let user = user_resource.get().flatten();
                    let navigate = navigate.clone();
                    match user {
                        Some(user) if user.is_admin => {
                            view! {
                                <Suspense fallback=|| {
                                    view! { <div class="dark:text-gray-300">"Loading..."</div> }
                                }>
                                    {move || {
                                        let navigate = navigate.clone();
                                        match post_resource.get() {
                                            None => {
                                                view! { <p class="dark:text-gray-300">"Loading..."</p> }
                                                    .into_any()
                                            }
                                            Some(Err(e)) => {
                                                view! {
                                                    <div class="bg-red-100 dark:bg-red-900/30 border border-red-300 dark:border-red-700 text-red-700 dark:text-red-300 px-4 py-3 rounded-lg mb-6 flex items-center gap-2">
                                                        <span class="i-mdi-alert-circle text-lg"></span>
                                                        <p>"Error loading post: " {e.to_string()}</p>
                                                    </div>
                                                }
                                                    .into_any()
                                            }
                                            Some(Ok(_)) => {
                                                view! {
                                                    <div>
                                                        <h1 class="text-3xl font-bold mb-6 dark:text-white flex items-center gap-2">
                                                            <span class="i-mdi-pencil-outline text-primary-500 dark:text-primary-400"></span>
                                                            "Edit Post"
                                                        </h1>

                                                        {move || {
                                                            error
                                                                .get()
                                                                .map(|err| {
                                                                    view! {
                                                                        <div
                                                                            class="bg-red-100 dark:bg-red-900/30 border border-red-300 dark:border-red-700 text-red-700 dark:text-red-300 px-4 py-3 rounded-lg mb-6 flex items-center gap-2"
                                                                            role="alert"
                                                                        >
                                                                            <span class="i-mdi-alert-circle text-lg"></span>
                                                                            <span class="block sm:inline">{err}</span>
                                                                        </div>
                                                                    }
                                                                })
                                                        }}

                                                        <form
                                                            on:submit=move |ev: ev::SubmitEvent| {
                                                                ev.prevent_default();
                                                                if title.get().trim().is_empty()
                                                                    || content.get().trim().is_empty()
                                                                {
                                                                    set_error
                                                                        .set(Some("Title and content are required".to_string()));
                                                                    return;
                                                                }
                                                                let update_data = UpdatePostData {
                                                                    id: post_id(),
                                                                    title: title.get(),
                                                                    content: content.get(),
                                                                    published: published.get(),
                                                                };
                                                                let navigate = navigate.clone();
                                                                spawn_local(async move {
                                                                    let result = update_post(update_data).await;
                                                                    match result {
                                                                        Ok(post) => {
                                                                            navigate(&format!("/blog/{}", post.id), Default::default());
                                                                        }
                                                                        Err(e) => {
                                                                            set_error.set(Some(e.to_string()));
                                                                        }
                                                                    }
                                                                });
                                                            }
                                                            class="bg-white dark:bg-primary-800 p-6 rounded-lg shadow-lg dark:shadow-primary-900/30 border border-gray-100 dark:border-primary-700"
                                                        >
                                                            <div class="mb-4">
                                                                <label
                                                                    for="title"
                                                                    class="block text-gray-700 dark:text-gray-200 font-medium mb-2 flex items-center gap-1"
                                                                >
                                                                    <span class="i-mdi-format-title"></span>
                                                                    "Title"
                                                                </label>
                                                                <input
                                                                    type="text"
                                                                    id="title"
                                                                    class="w-full px-3 py-2 border border-gray-300 dark:border-primary-600 dark:bg-primary-700/50 dark:text-white rounded-lg focus:outline-none focus:ring-2 focus:ring-primary-500 focus:border-transparent transition-all duration-200"
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
                                                                    class="block text-gray-700 dark:text-gray-200 font-medium mb-2 flex items-center gap-1"
                                                                >
                                                                    <span class="i-mdi-file-document-outline"></span>
                                                                    "Content"
                                                                </label>
                                                                <textarea
                                                                    id="content"
                                                                    class="w-full px-3 py-2 border border-gray-300 dark:border-primary-600 dark:bg-primary-700/50 dark:text-white rounded-lg focus:outline-none focus:ring-2 focus:ring-primary-500 focus:border-transparent transition-all duration-200"
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
                                                                        class="form-checkbox h-5 w-5 text-primary-600 dark:text-primary-400 dark:border-primary-600 dark:bg-primary-700/50"
                                                                        on:input=move |ev| {
                                                                            set_published.set(event_target_checked(&ev));
                                                                        }
                                                                        prop:checked=published
                                                                    />
                                                                    <span class="ml-2 text-gray-700 dark:text-gray-200 flex items-center gap-1">
                                                                        <span class="i-mdi-earth"></span>
                                                                        "Published"
                                                                    </span>
                                                                </label>
                                                            </div>

                                                            <div class="flex justify-between">
                                                                <button
                                                                    type="submit"
                                                                    class="bg-gradient-to-r from-primary-500 to-accent-500 text-white py-2 px-4 rounded-lg hover:from-primary-600 hover:to-accent-600 focus:outline-none focus:ring-2 focus:ring-primary-500 focus:ring-offset-2 dark:focus:ring-offset-primary-800 transition-all duration-200 font-medium flex items-center gap-2"
                                                                >
                                                                    <span class="i-mdi-content-save"></span>
                                                                    "Update Post"
                                                                </button>

                                                                <A
                                                                    href=format!("/blog/{}", post_id())
                                                                    attr:class="py-2 px-4 border border-gray-300 dark:border-primary-600 rounded-lg hover:bg-gray-100 dark:hover:bg-primary-700 text-gray-700 dark:text-gray-200 transition-colors flex items-center gap-2"
                                                                >
                                                                    <span class="i-mdi-close-circle"></span>
                                                                    "Cancel"
                                                                </A>
                                                            </div>
                                                        </form>
                                                    </div>
                                                }
                                                    .into_any()
                                            }
                                        }
                                    }}
                                </Suspense>
                            }
                                .into_any()
                        }
                        _ => {
                            view! {
                                <div class="text-center py-12 bg-white/90 dark:bg-primary-800/90 p-8 rounded-xl shadow-lg dark:shadow-primary-900/50 backdrop-blur-sm border border-gray-100 dark:border-primary-700">
                                    <div class="inline-flex p-3 bg-red-100 dark:bg-red-900/30 rounded-full mb-6 text-red-500 dark:text-red-400">
                                        <span class="i-mdi-shield-alert text-3xl"></span>
                                    </div>
                                    <h1 class="text-2xl font-bold mb-4 dark:text-white">
                                        "Access Denied"
                                    </h1>
                                    <p class="mb-6 dark:text-gray-300">
                                        "You must be an admin to edit posts."
                                    </p>
                                    <A
                                        href="/login"
                                        attr:class="inline-flex items-center gap-2 bg-gradient-to-r from-primary-500 to-accent-500 text-white py-2 px-4 rounded-lg hover:from-primary-600 hover:to-accent-600 transition-all duration-200"
                                    >
                                        <span class="i-mdi-login"></span>
                                        "Login"
                                    </A>
                                </div>
                            }
                                .into_any()
                        }
                    }
                }}
            </Suspense>
        </div>
    }
}
