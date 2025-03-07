use crate::models::user::{LoginCredentials, User};
use crate::server::auth::login;
use leptos::{ev, prelude::*, task::spawn_local};
use leptos_router::components::A;

/// Login page component
#[component]
pub fn LoginPage() -> impl IntoView {
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

