use crate::app::CurrentUser;
use crate::components::theme_switcher::ThemeSwitcher;
use leptos::either::EitherOr;
use leptos::prelude::*;
use leptos_router::components::A;

use super::theme_switcher::ThemeProvider;

#[component]
pub fn Header() -> impl IntoView {
    let current_user = expect_context::<CurrentUser>();

    let not_auth_buttons = move || {
        view! {
            <div class="flex space-x-2 sm:space-x-4">
                <A
                    href="/login"
                    attr:class="px-3 sm:px-4 py-2 rounded-full bg-white/20 hover:bg-white/30 transition-all duration-300 font-medium text-sm sm:text-base flex items-center gap-1"
                >
                    <span class="i-mdi-login"></span>
                    <span class="hidden sm:inline">"Login"</span>
                </A>
                <A
                    href="/signup"
                    attr:class="px-3 sm:px-4 py-2 rounded-full bg-accent-500 hover:bg-accent-400 transition-all duration-300 shadow-sm font-medium text-sm sm:text-base flex items-center gap-1"
                >
                    <span class="i-mdi-account-plus"></span>
                    <span class="hidden sm:inline">"Signup"</span>
                </A>
            </div>
        }
    };

    let (mobile_menu_open, set_mobile_menu_open) = signal(false);

    let toggle_mobile_menu = move |_| {
        set_mobile_menu_open.update(|open| *open = !*open);
    };

    let mobile_menu_closed = move || !mobile_menu_open.get();

    view! {
        <ThemeProvider>
            <header class="bg-gradient-to-r from-primary-600 to-accent-600 text-white p-4 shadow-lg">
                <div class="container mx-auto flex justify-between items-center">
                    <A
                        href="/"
                        attr:class="text-xl sm:text-2xl font-bold hover-grow flex items-center gap-2"
                    >
                        <span class="i-mdi-code-tags text-2xl sm:text-3xl"></span>
                        "Léo Coletta"
                    </A>

                    // Mobile menu button
                    <button
                        class="lg:hidden flex items-center text-white focus:outline-none relative w-8 h-8 transition-all duration-300 hover:cursor-pointer"
                        on:click=toggle_mobile_menu
                    >
                        <span
                            class="i-mdi-menu text-2xl absolute transition-all duration-300"
                            class=("opacity-0 rotate-90 scale-50", mobile_menu_open)
                            class=("opacity-100 rotate-0 scale-100", mobile_menu_closed)
                        ></span>
                        <span
                            class="i-mdi-close text-2xl absolute transition-all duration-300"
                            class=("opacity-100 rotate-0 scale-100", mobile_menu_open)
                            class=("opacity-0 rotate-90 scale-50", mobile_menu_closed)
                        ></span>
                    </button>

                    // Desktop Navigation
                    <nav class="hidden lg:flex space-x-6 items-center">
                        <A
                            href="/"
                            attr:class="hover:text-white/80 font-medium transition-all duration-300 hover:-translate-y-1 flex items-center gap-1"
                        >
                            <span class="i-mdi-home"></span>
                            "Home"
                        </A>
                        <A
                            href="/blog"
                            attr:class="hover:text-white/80 font-medium transition-all duration-300 hover:-translate-y-1 flex items-center gap-1"
                        >
                            <span class="i-mdi-post"></span>
                            "Blog"
                        </A>
                        <a
                            href="https://github.com/leo91000"
                            target="_blank"
                            rel="noopener noreferrer"
                            class="hover:text-white/80 font-medium transition-all duration-300 hover:-translate-y-1 flex items-center gap-1"
                        >
                            <span class="i-mdi-github"></span>
                            "GitHub"
                        </a>

                        <Await future=current_user.into_future() let:server_user>
                            {server_user
                                .clone()
                                .either_or(
                                    |user| {
                                        view! {
                                            <div class="flex space-x-4 items-center">
                                                {if user.is_admin {
                                                    Some(
                                                        view! {
                                                            <A
                                                                href="/blog/new"
                                                                attr:class="px-3 sm:px-4 py-2 rounded-full bg-primary-500 hover:bg-primary-400 transition-all duration-300 shadow-sm font-medium flex items-center gap-1"
                                                            >
                                                                <span class="i-mdi-plus"></span>
                                                                "New Post"
                                                            </A>
                                                        },
                                                    )
                                                } else {
                                                    None
                                                }}
                                                <span class="font-medium text-white/90 mr-2">
                                                    {format!("Hi, {}", user.username)}
                                                </span>
                                                <A
                                                    href="/logout"
                                                    attr:class="px-3 py-1 rounded-full bg-white/20 hover:bg-white/30 transition-all duration-300 font-medium text-sm flex items-center gap-1"
                                                >
                                                    <span class="i-mdi-logout"></span>
                                                    "Logout"
                                                </A>
                                            </div>
                                        }
                                    },
                                    |_| not_auth_buttons(),
                                )}
                        </Await>

                        <ThemeSwitcher />
                    </nav>
                </div>

                // Mobile menu
                <div class=move || {
                    let base = "lg:hidden overflow-hidden transition-all duration-300 ease-in-out"
                        .to_string();
                    if mobile_menu_open.get() {
                        format!(
                            "{base} max-h-[500px] opacity-100 translate-y-0 scale-100 mt-4 border-t border-white/20 pt-4",
                        )
                    } else {
                        format!(
                            "{base} max-h-0 opacity-0 translate-y-[-10px] scale-95 pointer-events-none",
                        )
                    }
                }>

                    <nav class="flex flex-col space-y-4 items-center justify-center transition-transform duration-300">
                        <A
                            href="/"
                            attr:class=move || {
                                let mut base_class = "hover:text-white/80 font-medium flex items-center gap-2 p-2 transition-all duration-300"
                                    .to_string();
                                if mobile_menu_open.get() {
                                    base_class += " opacity-100 translate-y-0 delay-[0ms]";
                                } else {
                                    base_class += " opacity-0 translate-y-2";
                                }
                                base_class
                            }
                            on:click=move |_| set_mobile_menu_open.set(false)
                        >
                            <span class="i-mdi-home text-xl"></span>
                            "Home"
                        </A>
                        <A
                            href="/blog"
                            attr:class=move || {
                                let mut base_class = "hover:text-white/80 font-medium flex items-center gap-2 p-2 transition-all duration-300"
                                    .to_string();
                                if mobile_menu_open.get() {
                                    base_class += " opacity-100 translate-y-0 delay-75";
                                } else {
                                    base_class += " opacity-0 translate-y-2";
                                }
                                base_class
                            }
                            on:click=move |_| set_mobile_menu_open.set(false)
                        >
                            <span class="i-mdi-post text-xl"></span>
                            "Blog"
                        </A>
                        <a
                            href="https://github.com/leo91000"
                            target="_blank"
                            rel="noopener noreferrer"
                            class=move || {
                                let mut base_class = "hover:text-white/80 font-medium flex items-center gap-2 p-2 transition-all duration-300"
                                    .to_string();
                                if mobile_menu_open.get() {
                                    base_class += " opacity-100 translate-y-0 delay-150";
                                } else {
                                    base_class += " opacity-0 translate-y-2";
                                }
                                base_class
                            }
                            on:click=move |_| set_mobile_menu_open.set(false)
                        >
                            <span class="i-mdi-github text-xl"></span>
                            "GitHub"
                        </a>

                        <Await future=current_user.into_future() let:server_user>
                            {server_user
                                .clone()
                                .either_or(
                                    |user| {
                                        view! {
                                            <div class=move || {
                                                let mut base_class = "flex flex-col space-y-2 pt-4 border-t border-white/20 transition-all duration-300"
                                                    .to_string();
                                                if mobile_menu_open.get() {
                                                    base_class += " opacity-100 translate-y-0 delay-200";
                                                } else {
                                                    base_class += " opacity-0 translate-y-2";
                                                }
                                                base_class
                                            }>

                                                {if user.is_admin {
                                                    Some(
                                                        view! {
                                                            <A
                                                                href="/blog/new"
                                                                attr:class="px-4 py-2 rounded-full bg-primary-500 hover:bg-primary-400 transition-all duration-300 shadow-sm font-medium flex items-center gap-2 justify-center"
                                                                on:click=move |_| set_mobile_menu_open.set(false)
                                                            >
                                                                <span class="i-mdi-plus text-xl"></span>
                                                                "New Post"
                                                            </A>
                                                        },
                                                    )
                                                } else {
                                                    None
                                                }}
                                                <span class="font-medium text-white/90 px-2 py-1 text-center">
                                                    {format!("Hi, {}", user.username)}
                                                </span>
                                                <A
                                                    href="/logout"
                                                    attr:class="px-4 py-2 rounded-full bg-white/20 hover:bg-white/30 transition-all duration-300 font-medium flex items-center gap-2 justify-center"
                                                    on:click=move |_| set_mobile_menu_open.set(false)
                                                >
                                                    <span class="i-mdi-logout text-xl"></span>
                                                    "Logout"
                                                </A>
                                            </div>
                                        }
                                    },
                                    |_| {
                                        view! {
                                            <div class=move || {
                                                let mut base_class = "flex flex-col space-y-2 pt-4 border-t border-white/20 transition-all duration-300"
                                                    .to_string();
                                                if mobile_menu_open.get() {
                                                    base_class += " opacity-100 translate-y-0 delay-200";
                                                } else {
                                                    base_class += " opacity-0 translate-y-2";
                                                }
                                                base_class
                                            }>
                                                <A
                                                    href="/login"
                                                    attr:class="px-4 py-2 rounded-full bg-white/20 hover:bg-white/30 transition-all duration-300 font-medium flex items-center gap-2 justify-center"
                                                    on:click=move |_| set_mobile_menu_open.set(false)
                                                >
                                                    <span class="i-mdi-login text-xl"></span>
                                                    "Login"
                                                </A>
                                                <A
                                                    href="/signup"
                                                    attr:class="px-4 py-2 rounded-full bg-accent-500 hover:bg-accent-400 transition-all duration-300 shadow-sm font-medium flex items-center gap-2 justify-center"
                                                    on:click=move |_| set_mobile_menu_open.set(false)
                                                >
                                                    <span class="i-mdi-account-plus text-xl"></span>
                                                    "Signup"
                                                </A>
                                            </div>
                                        }
                                    },
                                )}
                        </Await>

                        <div class=move || {
                            let mut base_class = "flex justify-center pt-4 border-t border-white/20 transition-all duration-300"
                                .to_string();
                            if mobile_menu_open.get() {
                                base_class += " opacity-100 translate-y-0 delay-[250ms]";
                            } else {
                                base_class += " opacity-0 translate-y-2";
                            }
                            base_class
                        }>
                            <ThemeSwitcher />
                        </div>
                    </nav>
                </div>
            </header>
        </ThemeProvider>
    }
}
