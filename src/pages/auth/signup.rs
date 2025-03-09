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
        <div class="max-w-md mx-auto bg-white/90 dark:bg-primary-800/90 p-8 rounded-xl shadow-lg dark:shadow-primary-900/50 backdrop-blur-sm border border-gray-100 dark:border-primary-700 w-full">
            <div class="text-center mb-8">
                <div class="inline-flex p-2 bg-gradient-to-br from-primary-400 to-accent-500 rounded-full mb-4">
                    <span class="i-mdi-account-plus text-white h-8 w-8"></span>
                </div>
                <h1 class="text-3xl font-bold dark:text-white">"Create Account"</h1>
                <p class="text-gray-600 dark:text-gray-300 mt-2">"Join our community"</p>
            </div>

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

            {move || {
                if success.get() {
                    view! {
                        <div
                            class="bg-green-100 dark:bg-green-900/30 border border-green-300 dark:border-green-700 text-green-700 dark:text-green-300 px-4 py-3 rounded-lg mb-6 flex items-center gap-2"
                            role="alert"
                        >
                            <span class="i-mdi-check-circle text-lg"></span>
                            <span class="block sm:inline">
                                "Registration successful! You can now "
                                <A
                                    href="/login"
                                    attr:class="text-primary-600 dark:text-primary-400 hover:underline font-medium"
                                >
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

            <form on:submit=on_submit class="space-y-6">
                <div>
                    <label
                        for="username"
                        class="block text-gray-700 dark:text-gray-200 font-medium mb-2 flex items-center gap-1"
                    >
                        <span class="i-mdi-account-outline"></span>
                        "Username"
                    </label>
                    <div class="relative">
                        <span class="absolute left-3 inset-y-0 flex items-center text-gray-400 dark:text-gray-300">
                            <span class="i-mdi-account text-lg"></span>
                        </span>
                        <input
                            type="text"
                            id="username"
                            class="w-full pl-10 pr-3 py-2 border border-gray-300 dark:border-primary-600 dark:bg-primary-700/50 dark:text-white rounded-lg focus:outline-none focus:ring-2 focus:ring-primary-500 focus:border-transparent transition-all duration-200"
                            placeholder="Choose a username"
                            on:input=move |ev| {
                                set_username.set(event_target_value(&ev));
                            }
                            prop:value=username
                        />
                    </div>
                </div>

                <div>
                    <label
                        for="password"
                        class="block text-gray-700 dark:text-gray-200 font-medium mb-2 flex items-center gap-1"
                    >
                        <span class="i-mdi-lock-outline"></span>
                        "Password"
                    </label>
                    <div class="relative">
                        <span class="absolute left-3 inset-y-0 flex items-center text-gray-400 dark:text-gray-300">
                            <span class="i-mdi-lock text-lg"></span>
                        </span>
                        <input
                            type="password"
                            id="password"
                            class="w-full pl-10 pr-3 py-2 border border-gray-300 dark:border-primary-600 dark:bg-primary-700/50 dark:text-white rounded-lg focus:outline-none focus:ring-2 focus:ring-primary-500 focus:border-transparent transition-all duration-200"
                            placeholder="Create a password"
                            on:input=move |ev| {
                                set_password.set(event_target_value(&ev));
                            }
                            prop:value=password
                        />
                    </div>
                </div>

                <div>
                    <label
                        for="confirm-password"
                        class="block text-gray-700 dark:text-gray-200 font-medium mb-2 flex items-center gap-1"
                    >
                        <span class="i-mdi-lock-check-outline"></span>
                        "Confirm Password"
                    </label>
                    <div class="relative">
                        <span class="absolute left-3 inset-y-0 flex items-center text-gray-400 dark:text-gray-300">
                            <span class="i-mdi-lock-check text-lg"></span>
                        </span>
                        <input
                            type="password"
                            id="confirm-password"
                            class="w-full pl-10 pr-3 py-2 border border-gray-300 dark:border-primary-600 dark:bg-primary-700/50 dark:text-white rounded-lg focus:outline-none focus:ring-2 focus:ring-primary-500 focus:border-transparent transition-all duration-200"
                            placeholder="Confirm your password"
                            on:input=move |ev| {
                                set_confirm_password.set(event_target_value(&ev));
                            }
                            prop:value=confirm_password
                        />
                    </div>
                </div>

                <button
                    type="submit"
                    class="w-full bg-gradient-to-r from-primary-500 to-accent-500 text-white py-3 px-4 rounded-lg hover:from-primary-600 hover:to-accent-600 focus:outline-none focus:ring-2 focus:ring-primary-500 focus:ring-offset-2 dark:focus:ring-offset-primary-800 transition-all duration-200 font-medium flex items-center justify-center gap-2"
                >
                    <span class="i-mdi-account-plus"></span>
                    "Sign Up"
                </button>
            </form>

            <p class="mt-6 text-center dark:text-gray-300 text-sm">
                "Already have an account? "
                <A
                    href="/login"
                    attr:class="text-primary-600 dark:text-primary-400 hover:underline font-medium"
                >
                    "Login"
                </A>
            </p>
        </div>
    }
}
