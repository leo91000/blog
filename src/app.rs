use crate::models::user::User;
use crate::server::auth::get_current_user;
use crate::{components::header::Header, models::post::Post};
use leptos::{ev, prelude::*, task::spawn_local};
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes, A},
    StaticSegment,
};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <AutoReload options=options.clone() />
                <HydrationScripts options />
                <MetaTags />
            </head>
            <body class="bg-gray-50 min-h-screen flex flex-col">
                <App />
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    // Get current user
    let user_resource = OnceResource::new(async move { get_current_user().await.ok().flatten() });

    let current_user = move || user_resource.get().flatten();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/blog.css" />

        // sets the document title
        <Title text="Léo Coletta - Software Engineer" />

        // content for this welcome page
        <Router>
            // Show header with current user
            <Header current_user=Signal::derive(current_user) />

            <main class="container mx-auto py-8 px-4 flex-grow">
                <Routes fallback=|| {
                    view! {
                        <div class="text-center py-12">
                            <h1 class="text-2xl font-bold">Page not found.</h1>
                        </div>
                    }
                }>
                    <Route path=StaticSegment("") view=HomePage />
                    <Route path=StaticSegment("login") view=LoginPage />
                    <Route path=StaticSegment("signup") view=SignupPage />
                    <Route path=StaticSegment("logout") view=LogoutPage />
                </Routes>
            </main>

            <footer class="bg-gray-800 text-white p-4 mt-auto">
                <div class="container mx-auto text-center">
                    <p>"© 2025 Léo Coletta. All rights reserved."</p>
                </div>
            </footer>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    view! {
        <div class="max-w-3xl mx-auto">
            <h1 class="text-4xl font-bold mb-6">"Léo Coletta"</h1>
            <h2 class="text-2xl font-semibold mb-4">"Software Engineer"</h2>

            <div class="prose max-w-none">
                <p class="mb-4">
                    "Welcome to my personal website! I'm a passionate software engineer specializing in Rust,
                    web technologies, and distributed systems."
                </p>

                <p class="mb-6">
                    "Check out my " <A href="/blog" attr:class="text-blue-600 hover:underline">
                        "blog"
                    </A>
                    " where I share technical tutorials, insights on programming, and my experiences in the tech industry."
                </p>
            </div>

            <div class="mt-8">
                <h3 class="text-xl font-semibold mb-3">"Recent Projects"</h3>
                <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                    <div class="bg-white p-4 rounded shadow">
                        <h4 class="font-bold">"Rust Blog Engine"</h4>
                        <p>"A blazing fast blog engine built with Rust and Leptos."</p>
                    </div>
                    <div class="bg-white p-4 rounded shadow">
                        <h4 class="font-bold">"Distributed Cache"</h4>
                        <p>"High-performance distributed caching system written in Rust."</p>
                    </div>
                </div>
            </div>
        </div>
    }
}

/// Login page component
#[component]
fn LoginPage() -> impl IntoView {
    use crate::models::user::LoginCredentials;
    use crate::server::auth::login;

    let (username, set_username) = signal(String::new());
    let (password, set_password) = signal(String::new());
    let (error, set_error) = signal(Option::<String>::None);

    // Navigate after successful login
    let navigate = leptos_router::hooks::use_navigate();

    // Login result storage
    let (login_result, set_login_result) = signal(None::<Result<User, ServerFnError>>);

    let on_submit = move |ev: ev::SubmitEvent| {
        ev.prevent_default();

        // Validate inputs
        if username.get_untracked().trim().is_empty() || password.get_untracked().trim().is_empty()
        {
            set_error.set(Some("Username and password are required".to_string()));
            return;
        }

        // Clear previous error
        set_error.set(None);

        // Create login credentials
        let credentials = LoginCredentials {
            username: username.get(),
            password: password.get(),
        };

        // Use spawn_local instead of create_server_action
        spawn_local(async move {
            let result = login(credentials).await;
            set_login_result.set(Some(result));
        });
    };

    // Handle action response
    Effect::new(move |_| {
        if let Some(result) = login_result.get() {
            match result {
                Ok(_) => {
                    // Redirect to home on success
                    navigate("/", Default::default());
                }
                Err(e) => {
                    // Show error message
                    set_error.set(Some(e.to_string()));
                }
            }
        }
    });

    view! {
        <div class="max-w-md mx-auto bg-white p-8 rounded shadow">
            <h1 class="text-2xl font-bold mb-6">"Login"</h1>

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

            <form on:submit=on_submit>
                <div class="mb-4">
                    <label for="username" class="block text-gray-700 font-bold mb-2">
                        "Username"
                    </label>
                    <input
                        type="text"
                        id="username"
                        class="w-full px-3 py-2 border border-gray-300 rounded focus:outline-none focus:border-blue-500"
                        placeholder="Enter your username"
                        on:input=move |ev| {
                            set_username.set(event_target_value(&ev));
                        }
                        prop:value=username
                    />
                </div>

                <div class="mb-6">
                    <label for="password" class="block text-gray-700 font-bold mb-2">
                        "Password"
                    </label>
                    <input
                        type="password"
                        id="password"
                        class="w-full px-3 py-2 border border-gray-300 rounded focus:outline-none focus:border-blue-500"
                        placeholder="Enter your password"
                        on:input=move |ev| {
                            set_password.set(event_target_value(&ev));
                        }
                        prop:value=password
                    />
                </div>

                <button
                    type="submit"
                    class="w-full bg-blue-500 text-white py-2 px-4 rounded hover:bg-blue-600 focus:outline-none focus:bg-blue-600"
                >
                    "Login"
                </button>
            </form>

            <p class="mt-4 text-center">
                "Don't have an account? "
                <A href="/signup" attr:class="text-blue-500 hover:underline">
                    "Sign up"
                </A>
            </p>
        </div>
    }
}

/// Signup page component
#[component]
fn SignupPage() -> impl IntoView {
    use crate::models::user::NewUser;
    use crate::server::auth::register;

    let (username, set_username) = signal(String::new());
    let (password, set_password) = signal(String::new());
    let (confirm_password, set_confirm_password) = signal(String::new());
    let (error, set_error) = signal(Option::<String>::None);
    let (success, set_success) = signal(false);

    // Signup result storage
    let (register_result, set_register_result) = signal(None::<Result<(), ServerFnError>>);

    let on_submit = move |ev: ev::SubmitEvent| {
        ev.prevent_default();

        // Validate inputs
        if username.get_untracked().trim().is_empty() || password.get_untracked().trim().is_empty()
        {
            set_error.set(Some("All fields are required".to_string()));
            return;
        }

        if password.get_untracked() != confirm_password.get_untracked() {
            set_error.set(Some("Passwords do not match".to_string()));
            return;
        }

        if password.get_untracked().len() < 8 {
            set_error.set(Some("Password must be at least 8 characters".to_string()));
            return;
        }

        // Clear previous error
        set_error.set(None);

        // Create new user
        let new_user = NewUser {
            username: username.get_untracked(),
            password: password.get_untracked(),
        };

        // Use spawn_local instead of create_server_action
        spawn_local(async move {
            let result = register(new_user).await;
            set_register_result.set(Some(result));
        });
    };

    // Handle action response
    Effect::new(move |_| {
        if let Some(result) = register_result.get() {
            match result {
                Ok(_) => {
                    // Show success message
                    set_success.set(true);
                    set_username.set(String::new());
                    set_password.set(String::new());
                    set_confirm_password.set(String::new());
                }
                Err(e) => {
                    // Show error message
                    set_error.set(Some(e.to_string()));
                }
            }
        }
    });

    view! {
        <div class="max-w-md mx-auto bg-white p-8 rounded shadow">
            <h1 class="text-2xl font-bold mb-6">"Sign Up"</h1>

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

            {move || {
                if success.get() {
                    view! {
                        <div
                            class="bg-green-100 border border-green-400 text-green-700 px-4 py-3 rounded mb-4"
                            role="alert"
                        >
                            <span class="block sm:inline">
                                "Registration successful! You can now "
                                <A href="/login" attr:class="underline">
                                    "login"
                                </A> " to your account."
                            </span>
                        </div>
                    }
                        .into_any()
                } else {
                    ().into_any()
                }
            }}

            <form on:submit=on_submit>
                <div class="mb-4">
                    <label for="username" class="block text-gray-700 font-bold mb-2">
                        "Username"
                    </label>
                    <input
                        type="text"
                        id="username"
                        class="w-full px-3 py-2 border border-gray-300 rounded focus:outline-none focus:border-blue-500"
                        placeholder="Choose a username"
                        on:input=move |ev| {
                            set_username.set(event_target_value(&ev));
                        }
                        prop:value=username
                    />
                </div>

                <div class="mb-4">
                    <label for="password" class="block text-gray-700 font-bold mb-2">
                        "Password"
                    </label>
                    <input
                        type="password"
                        id="password"
                        class="w-full px-3 py-2 border border-gray-300 rounded focus:outline-none focus:border-blue-500"
                        placeholder="Create a password"
                        on:input=move |ev| {
                            set_password.set(event_target_value(&ev));
                        }
                        prop:value=password
                    />
                </div>

                <div class="mb-6">
                    <label for="confirm-password" class="block text-gray-700 font-bold mb-2">
                        "Confirm Password"
                    </label>
                    <input
                        type="password"
                        id="confirm-password"
                        class="w-full px-3 py-2 border border-gray-300 rounded focus:outline-none focus:border-blue-500"
                        placeholder="Confirm your password"
                        on:input=move |ev| {
                            set_confirm_password.set(event_target_value(&ev));
                        }
                        prop:value=confirm_password
                    />
                </div>

                <button
                    type="submit"
                    class="w-full bg-blue-500 text-white py-2 px-4 rounded hover:bg-blue-600 focus:outline-none focus:bg-blue-600"
                >
                    "Sign Up"
                </button>
            </form>

            <p class="mt-4 text-center">
                "Already have an account? "
                <A href="/login" attr:class="text-blue-500 hover:underline">
                    "Login"
                </A>
            </p>
        </div>
    }
}

/// Logout page component
#[component]
fn LogoutPage() -> impl IntoView {
    use crate::server::auth::logout;

    // Navigate after successful logout
    let navigate = leptos_router::hooks::use_navigate();

    // Execute logout server function directly
    spawn_local(async move {
        if (logout().await).is_ok() {
            // Redirect to home
            navigate("/", Default::default());
        }
    });

    view! {
        <div class="text-center py-8">
            <p>"Logging out..."</p>
        </div>
    }
}

/// Blog listing page
#[component]
fn BlogPage() -> impl IntoView {
    use crate::models::post::Post;
    use crate::server::blog::get_posts;

    // Create resource to fetch posts
    let posts_resource = OnceResource::new(async move { get_posts(true).await });

    // Format date for display
    let format_date = |date: chrono::DateTime<chrono::Utc>| date.format("%B %d, %Y").to_string();

    view! {
        <div class="max-w-4xl mx-auto">
            <h1 class="text-3xl font-bold mb-8">"Blog Posts"</h1>

            {move || match posts_resource.get() {
                None => view! { <p>"Loading..."</p> }.into_any(),
                Some(Err(e)) => {
                    view! { <p class="text-red-500">"Error loading posts: " {e.to_string()}</p> }
                        .into_any()
                }
                Some(Ok(posts)) => {
                    if posts.is_empty() {
                        view! { <p>"No posts available yet."</p> }.into_any()
                    } else {
                        view! {
                            <div class="space-y-8">
                                <For
                                    each=move || posts.clone()
                                    key=|post| post.id
                                    children=move |post: Post| {
                                        view! {
                                            <article class="bg-white p-6 rounded shadow">
                                                <h2 class="text-2xl font-bold mb-2">
                                                    <A
                                                        href=format!("/blog/{}", post.id)
                                                        attr:class="hover:text-blue-600"
                                                    >
                                                        {post.title.clone()}
                                                    </A>
                                                </h2>
                                                <div class="text-gray-500 mb-4">
                                                    {format_date(post.created_at)}
                                                </div>
                                                <div class="prose">
                                                    // Display a preview of the content
                                                    {post.content.chars().take(200).collect::<String>()}
                                                    {if post.content.len() > 200 { "..." } else { "" }}
                                                </div>
                                                <div class="mt-4">
                                                    <A
                                                        href=format!("/blog/{}", post.id)
                                                        attr:class="text-blue-600 hover:underline"
                                                    >
                                                        "Read more →"
                                                    </A>
                                                </div>
                                            </article>
                                        }
                                    }
                                />
                            </div>
                        }
                            .into_any()
                    }
                }
            }}
        </div>
    }
}

/// Single post page
#[component]
fn PostPage() -> impl IntoView {
    use crate::server::auth::get_current_user;
    use crate::server::blog::{delete_post, get_post};
    use leptos_router::hooks::use_params_map;

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

/// New post page
#[component]
fn NewPostPage() -> impl IntoView {
    use crate::models::post::NewPost;
    use crate::server::auth::get_current_user;
    use crate::server::blog::create_post;

    // Check if user is admin
    let user_resource =
        OnceResource::new(async move { (get_current_user().await).unwrap_or_default() });

    let (title, set_title) = signal(String::new());
    let (content, set_content) = signal(String::new());
    let (published, set_published) = signal(false);
    let (error, set_error) = signal(Option::<String>::None);

    // Create result storage
    let (create_result, set_create_result) = signal(None::<Result<Post, ServerFnError>>);
    let navigate = leptos_router::hooks::use_navigate();

    let on_submit = move |ev: ev::SubmitEvent| {
        ev.prevent_default();

        // Validate inputs
        if title.get_untracked().trim().is_empty() || content.get_untracked().trim().is_empty() {
            set_error.set(Some("Title and content are required".to_string()));
            return;
        }

        // Create new post
        let new_post = NewPost {
            title: title.get_untracked(),
            content: content.get_untracked(),
            published: published.get_untracked(),
        };

        spawn_local(async move {
            let result = create_post(new_post).await;
            set_create_result.set(Some(result));
        });
    };

    // Handle action result
    Effect::new(move |_| {
        if let Some(result) = create_result.get() {
            match result {
                Ok(post) => {
                    // Redirect to the new post
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
                            <div>
                                <h1 class="text-3xl font-bold mb-6">"Create New Post"</h1>

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

                                <form on:submit=on_submit class="bg-white p-6 rounded shadow">
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
                                            <span class="ml-2 text-gray-700">"Publish now"</span>
                                        </label>
                                    </div>

                                    <div class="flex justify-between">
                                        <button
                                            type="submit"
                                            class="bg-blue-500 text-white py-2 px-4 rounded hover:bg-blue-600 focus:outline-none focus:bg-blue-600"
                                        >
                                            "Create Post"
                                        </button>

                                        <A
                                            href="/blog"
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
                    _ => {
                        view! {
                            <div class="text-center py-12">
                                <h1 class="text-2xl font-bold mb-4">"Access Denied"</h1>
                                <p class="mb-6">"You must be an admin to create posts."</p>
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

/// Edit post page
#[component]
fn EditPostPage() -> impl IntoView {
    use crate::models::post::{Post, UpdatePostData};
    use crate::server::auth::get_current_user;
    use crate::server::blog::{get_post, update_post};
    use leptos_router::hooks::use_params_map;

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
