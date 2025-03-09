use crate::components::header::Header;
use crate::models::user::User;
use crate::pages::*;
use crate::server::auth::get_current_user;
use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    path, SsrMode,
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

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/blog.css" />

        // sets the document title
        <Title text="Léo Coletta - Software Engineer" />

        // Wrap the entire app with the ThemeProvider
        <Router>
            <Header />
            <main class="container mx-auto py-8 px-4 flex-grow flex flex-col">
                <div class="max-w-5xl mx-auto dark:text-white rounded-xl p-6 flex-grow h-full w-full items-center justify-center flex flex-col">
                    <Routes fallback=|| {
                        view! {
                            <div class="text-center py-12">
                                <h1 class="text-2xl font-bold text-primary-800 dark:text-primary-100">
                                    "Page not found."
                                </h1>
                                <p class="mt-4 text-gray-600 dark:text-gray-300">
                                    "The page you're looking for doesn't exist."
                                </p>
                            </div>
                        }
                    }>
                        <Route path=path!("") view=HomePage ssr=SsrMode::Async />
                        <Route path=path!("login") view=LoginPage ssr=SsrMode::Async />
                        <Route path=path!("signup") view=SignupPage ssr=SsrMode::Async />
                        <Route path=path!("logout") view=LogoutPage ssr=SsrMode::Async />
                        <Route path=path!("blog") view=BlogPage ssr=SsrMode::Async />
                        <Route path=path!("blog/new") view=NewPostPage ssr=SsrMode::Async />
                        <Route path=path!("blog/:id") view=PostPage ssr=SsrMode::Async />
                        <Route path=path!("blog/:id/edit") view=EditPostPage ssr=SsrMode::Async />
                    </Routes>
                </div>
            </main>
            <footer class="bg-gradient-to-r from-primary-700 to-accent-700 dark:from-primary-800 dark:to-accent-800 text-white p-4 mt-auto shadow-inner">
                <div class="container mx-auto text-center">
                    <p>"© 2025 Léo Coletta. All rights reserved."</p>
                </div>
            </footer>
        </Router>
    }
}
