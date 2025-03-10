use crate::app::CurrentUser;
use crate::components::theme_switcher::ThemeSwitcher;
use leptos::prelude::*;
use leptos_router::components::A;

use super::theme_switcher::ThemeProvider;

#[component]
pub fn Header() -> impl IntoView {
    let current_user = expect_context::<CurrentUser>();

    let not_auth_buttons = || {
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
                            class:opacity-0=move || mobile_menu_open.get()
                            class:rotate-90=move || mobile_menu_open.get()
                            class:scale-50=move || mobile_menu_open.get()
                            class:opacity-100=move || !mobile_menu_open.get()
                            class:rotate-0=move || !mobile_menu_open.get()
                            class:scale-100=move || !mobile_menu_open.get()
                        ></span>
                        <span
                            class="i-mdi-close text-2xl absolute transition-all duration-300"
                            class:opacity-100=move || mobile_menu_open.get()
                            class:rotate-0=move || mobile_menu_open.get()
                            class:scale-100=move || mobile_menu_open.get()
                            class:opacity-0=move || !mobile_menu_open.get()
                            class:-rotate-90=move || !mobile_menu_open.get()
                            class:scale-50=move || !mobile_menu_open.get()
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

                        <Await future=current_user.into_future() let:_server_user>
                            <Show
                                when=move || current_user.get().flatten().is_some()
                                fallback=not_auth_buttons
                            >
                                <div class="flex space-x-4 items-center">
                                    {if current_user.get().flatten().is_some_and(|user| user.is_admin) {
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
                                        {format!(
                                            "Hi, {}",
                                            current_user
                                                .get()
                                                .flatten()
                                                .map(|user| user.username)
                                                .unwrap_or_default(),
                                        )}
                                    </span>
                                    <A
                                        href="/logout"
                                        attr:class="px-3 py-1 rounded-full bg-white/20 hover:bg-white/30 transition-all duration-300 font-medium text-sm flex items-center gap-1"
                                    >
                                        <span class="i-mdi-logout"></span>
                                        "Logout"
                                    </A>
                                </div>
                            </Show>
                        </Await>

                        <ThemeSwitcher />
                    </nav>
                </div>

                // Mobile menu
                <div
                    class=move || {
                        let base = "lg:hidden overflow-hidden transition-all duration-300 ease-in-out".to_string();
                        if mobile_menu_open.get() {
                            // When open: Show the menu with full height
                            format!("{} max-h-[500px] opacity-100 translate-y-0 scale-100 mt-4 border-t border-white/20 pt-4", base)
                        } else {
                            // When closed: Keep the container but make it invisible and without height
                            format!("{} max-h-0 opacity-0 translate-y-[-10px] scale-95 pointer-events-none", base)
                        }
                    }
                >

                    <nav class="flex flex-col space-y-4 items-center justify-center transition-transform duration-300">
                        <A
                            href="/"
                            attr:class=move || {
                                let mut base_class = "hover:text-white/80 font-medium flex items-center gap-2 p-2 transition-all duration-300".to_string();
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
                                let mut base_class = "hover:text-white/80 font-medium flex items-center gap-2 p-2 transition-all duration-300".to_string();
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
                                let mut base_class = "hover:text-white/80 font-medium flex items-center gap-2 p-2 transition-all duration-300".to_string();
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

                        <Await future=current_user.into_future() let:_server_user>
                            <Show
                                when=move || current_user.get().flatten().is_some()
                                fallback=move || {
                                    view! {
                                        <div class=move || {
                                            let mut base_class = "flex flex-col space-y-2 pt-4 border-t border-white/20 transition-all duration-300".to_string();
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
                                }
                            >
                                <div class=move || {
                                    let mut base_class = "flex flex-col space-y-2 pt-4 border-t border-white/20 transition-all duration-300".to_string();
                                    if mobile_menu_open.get() {
                                        base_class += " opacity-100 translate-y-0 delay-200";
                                    } else {
                                        base_class += " opacity-0 translate-y-2";
                                    }
                                    base_class
                                }>
                                    {if current_user.get().flatten().is_some_and(|user| user.is_admin) {
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
                                        {format!(
                                            "Hi, {}",
                                            current_user
                                                .get()
                                                .flatten()
                                                .map(|user| user.username)
                                                .unwrap_or_default(),
                                        )}
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
                            </Show>
                        </Await>

                        <div
                            class=move || {
                                let mut base_class = "flex justify-center pt-4 border-t border-white/20 transition-all duration-300".to_string();
                                if mobile_menu_open.get() {
                                    base_class += " opacity-100 translate-y-0 delay-[250ms]";
                                } else {
                                    base_class += " opacity-0 translate-y-2";
                                }
                                base_class
                            }
                        >
                            <ThemeSwitcher />
                        </div>
                    </nav>
                </div>
            </header>
        </ThemeProvider>
    }
}
