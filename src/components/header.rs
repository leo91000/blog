use crate::models::user::User;
use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn Header(#[prop(into)] current_user: Signal<Option<User>>) -> impl IntoView {
    let current_user = move || current_user.get();

    view! {
        <header class="bg-gray-800 text-white p-4">
            <div class="container mx-auto flex justify-between items-center">
                <A href="/" attr:class="text-2xl font-bold">
                    "Léo Coletta"
                </A>

                <nav class="flex space-x-4">
                    <A href="/" attr:class="hover:text-gray-300">
                        "Home"
                    </A>
                    <A href="/blog" attr:class="hover:text-gray-300">
                        "Blog"
                    </A>

                    <Show
                        when=move || current_user().is_some()
                        fallback=|| {
                            view! {
                                <div class="flex space-x-4">
                                    <A href="/login" attr:class="hover:text-gray-300">
                                        "Login"
                                    </A>
                                    <A href="/signup" attr:class="hover:text-gray-300">
                                        "Signup"
                                    </A>
                                </div>
                            }
                        }
                    >
                        <div class="flex space-x-4 items-center">
                            {if current_user().unwrap().is_admin {
                                Some(
                                    view! {
                                        <A href="/blog/new" attr:class="hover:text-gray-300">
                                            "New Post"
                                        </A>
                                    },
                                )
                            } else {
                                None
                            }}
                            <span class="text-gray-300">
                                {format!("Hi, {}", current_user().unwrap().username)}
                            </span> <A href="/logout" attr:class="hover:text-gray-300">
                                "Logout"
                            </A>
                        </div>
                    </Show>
                </nav>
            </div>
        </header>
    }
}
