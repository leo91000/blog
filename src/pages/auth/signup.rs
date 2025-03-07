use crate::models::user::NewUser;
use crate::server::auth::register;
use leptos::{ev, prelude::*, task::spawn_local};
use leptos_router::components::A;

/// Signup page component
#[component]
pub fn SignupPage() -> impl IntoView {
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

