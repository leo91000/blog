use crate::app::CurrentUser;
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
            username: username.get_untracked(),
            password: password.get_untracked(),
        };

        spawn_local(async move {
            let result = login(credentials).await;
            set_login_result.set(Some(result));
        });
    };

    // Get user resource from context to refresh after login
    let user_resource = expect_context::<CurrentUser>();

    // Handle action response
    Effect::new(move |_| {
        if let Some(result) = login_result.get() {
            match result {
                Ok(_) => {
                    // Refresh user resource to update the UI
                    user_resource.refetch();

                    // Redirect to home on success
                    navigate("/", Default::default());
                }
                Err(e) => {
                    // Show error message
                    set_error.set(Some(e.to_string()));
                    user_resource.set(None);
                }
            }
        }
    });

    view! {
        <div class="max-w-md mx-auto bg-white/90 dark:bg-primary-800/90 p-8 rounded-xl shadow-lg dark:shadow-primary-900/50 backdrop-blur-sm border border-gray-100 dark:border-primary-700 w-full">
            <div class="text-center mb-8">
                <div class="inline-flex p-2 bg-gradient-to-br from-primary-400 to-accent-500 rounded-full mb-4">
                    <span class="i-mdi-account text-white h-8 w-8"></span>
                </div>
                <h1 class="text-3xl font-bold dark:text-white">"Welcome Back"</h1>
                <p class="text-gray-600 dark:text-gray-300 mt-2">"Login to your account"</p>
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
                            placeholder="Enter your username"
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
                            placeholder="Enter your password"
                            on:input=move |ev| {
                                set_password.set(event_target_value(&ev));
                            }
                            prop:value=password
                        />
                    </div>
                </div>

                <button
                    type="submit"
                    class="w-full bg-gradient-to-r from-primary-500 to-accent-500 text-white py-3 px-4 rounded-lg hover:from-primary-600 hover:to-accent-600 focus:outline-none focus:ring-2 focus:ring-primary-500 focus:ring-offset-2 dark:focus:ring-offset-primary-800 transition-all duration-200 font-medium flex items-center justify-center gap-2"
                >
                    <span class="i-mdi-login"></span>
                    "Login"
                </button>
            </form>

            <p class="mt-6 text-center dark:text-gray-300 text-sm">
                "Don't have an account? "
                <A
                    href="/signup"
                    attr:class="text-primary-600 dark:text-primary-400 hover:underline font-medium"
                >
                    "Sign up"
                </A>
            </p>
        </div>
    }
}
