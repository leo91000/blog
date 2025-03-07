use crate::components::header::Header;
use crate::pages::*;
use crate::server::auth::get_current_user;
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
            <body class="bg-gray-50 min-h-screen flex flex-col">
                <App />
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    // Get current user
    let user_resource = OnceResource::new(async move { get_current_user().await.ok().flatten() });

    let current_user = move || user_resource.get().flatten();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/blog.css" />

        // sets the document title
        <Title text="Léo Coletta - Software Engineer" />

        // content for this welcome page
        <Router>
            // Show header with current user
            <Header current_user=Signal::derive(current_user) />

            <main class="container mx-auto py-8 px-4 flex-grow">
                <Routes fallback=|| {
                    view! {
                        <div class="text-center py-12">
                            <h1 class="text-2xl font-bold">Page not found.</h1>
                        </div>
                    }
                }>
                    <Route path=path!("") view=HomePage />
                    <Route path=path!("login") view=LoginPage />
                    <Route path=path!("signup") view=SignupPage />
                    <Route path=path!("logout") view=LogoutPage />
                    <Route path=path!("blog") view=BlogPage />
                    <Route path=path!("blog/:id") view=PostPage />
                    <Route path=path!("blog/new") view=NewPostPage />
                    <Route path=path!("blog/:id/edit") view=EditPostPage />
                </Routes>
            </main>

            <footer class="bg-gray-800 text-white p-4 mt-auto">
                <div class="container mx-auto text-center">
                    <p>"© 2025 Léo Coletta. All rights reserved."</p>
                </div>
            </footer>
        </Router>
    }
}