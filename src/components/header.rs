use crate::app::CurrentUser;
use crate::components::theme_switcher::ThemeSwitcher;
use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn Header() -> impl IntoView {
    let current_user = expect_context::<CurrentUser>();

    let not_auth_buttons = || {
        view! {
            <div class="flex space-x-4">
                <A href="/login" attr:class="px-4 py-2 rounded-full bg-white/20 hover:bg-white/30 transition-all duration-300 font-medium">
                    "Login"
                </A>
                <A href="/signup" attr:class="px-4 py-2 rounded-full bg-accent-500 hover:bg-accent-400 transition-all duration-300 shadow-sm font-medium">
                    "Signup"
                </A>
            </div>
        }
    };

    view! {
        <header class="bg-gradient-to-r from-primary-600 to-accent-600 text-white p-4 shadow-lg">
            <div class="container mx-auto flex justify-between items-center">
                <A href="/" attr:class="text-2xl font-bold hover-grow">
                    "Léo Coletta"
                </A>

                <nav class="flex space-x-6 items-center">
                    <A href="/" attr:class="hover:text-white/80 font-medium transition-all duration-300 hover:-translate-y-1 flex items-center">
                        "Home"
                    </A>
                    <A href="/blog" attr:class="hover:text-white/80 font-medium transition-all duration-300 hover:-translate-y-1 flex items-center">
                        "Blog"
                    </A>
                    
                    <ThemeSwitcher />

                    <Suspense fallback=not_auth_buttons>
                        {move || Suspend::new(async move {
                            current_user.await;

                            view! {
                                <Show
                                    when=move || current_user.get().flatten().is_some()
                                    fallback=not_auth_buttons
                                >
                                    <div class="flex space-x-4 items-center">
                                        {if current_user
                                            .get()
                                            .flatten()
                                            .map(|user| user.is_admin)
                                            .unwrap_or(false)
                                        {
                                            Some(
                                                view! {
                                                    <A href="/blog/new" attr:class="px-4 py-2 rounded-full bg-primary-500 hover:bg-primary-400 transition-all duration-300 shadow-sm font-medium flex items-center">
                                                        "New Post"
                                                    </A>
                                                },
                                            )
                                        } else {
                                            None
                                        }}
                                        <span class="font-medium text-white/90 mr-2">
                                            {format!(
                                                "Hi, {}",
                                                current_user
                                                    .get()
                                                    .flatten()
                                                    .map(|user| user.username)
                                                    .unwrap_or_default(),
                                            )}
                                        </span> <A href="/logout" attr:class="px-3 py-1 rounded-full bg-white/20 hover:bg-white/30 transition-all duration-300 font-medium text-sm">
                                            "Logout"
                                        </A>
                                    </div>
                                </Show>
                            }
                        })}
                    </Suspense>

                </nav>
            </div>
        </header>
    }
}
