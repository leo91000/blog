use crate::components::theme_switcher::use_theme_server_resource;
use crate::components::{header::Header, theme_switcher::use_color_theme};
use crate::models::session::ThemePreference;
use crate::models::user::User;
use crate::pages::*;
use crate::server::auth::get_current_user;
use cfg_if::cfg_if;
use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    path,
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
            <body class="bg-gradient-to-br from-primary-50 to-accent-50 dark:bg-gradient-to-br dark:from-primary-900 dark:to-accent-900 min-h-screen flex flex-col">
                <App />
            </body>
        </html>
    }
}

pub type CurrentUser = Resource<Option<User>>;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    // Get current user
    let user_resource = Resource::new(
        || (), // No dependencies
        async move |_| get_current_user().await.ok().flatten(),
    );

    // Provide user resource to the entire app via context
    provide_context(user_resource);

    let theme_server_resource = use_theme_server_resource();
    #[allow(unused)]
    let (theme, set_theme) = use_color_theme();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/blog.css" />

        // sets the document title
        <Title text="Léo Coletta - Software Engineer" />

        // content for this welcome page
        <Suspense fallback=|| {
            view! { <div>"Loading..."</div> }
        }>
            {move || Suspend::new(async move {
                cfg_if! {
                    if #[cfg(feature = "ssr")] {
                        let server_theme = theme_server_resource.await;
                        if let Ok(theme) = server_theme {
                            set_theme.set(theme);
                        }
                    } else {
                        theme_server_resource.await.ok();
                    }
                }

                view! {
                    <div
                        class:dark=theme.get() == ThemePreference::Dark
                        class="min-h-[100dvh] min-w-[100dvw] flex flex-col"
                    >
                        <Router>
                            <Header />
                            <main class="container mx-auto py-8 px-4 flex-grow">
                                <div class="max-w-5xl mx-auto bg-white/80 dark:bg-primary-800/80 dark:text-white backdrop-blur-sm rounded-xl shadow-xl p-6">
                                    <Routes fallback=|| {
                                        view! {
                                            <div class="text-center py-12">
                                                <h1 class="text-2xl font-bold text-primary-800">
                                                    "Page not found."
                                                </h1>
                                                <p class="mt-4 text-gray-600">
                                                    "The page you're looking for doesn't exist."
                                                </p>
                                            </div>
                                        }
                                    }>
                                        <Route path=path!("") view=HomePage />
                                        <Route path=path!("login") view=LoginPage />
                                        <Route path=path!("signup") view=SignupPage />
                                        <Route path=path!("logout") view=LogoutPage />
                                        <Route path=path!("blog") view=BlogPage />
                                        <Route path=path!("blog/new") view=NewPostPage />
                                        <Route path=path!("blog/:id") view=PostPage />
                                        <Route path=path!("blog/:id/edit") view=EditPostPage />
                                    </Routes>
                                </div>
                            </main>
                            <footer class="bg-gradient-to-r from-primary-700 to-accent-700 text-white p-4 mt-auto shadow-inner">
                                <div class="container mx-auto text-center">
                                    <p>"© 2025 Léo Coletta. All rights reserved."</p>
                                </div>
                            </footer>
                        </Router>
                    </div>
                }
            })}
        </Suspense>
    }
}
