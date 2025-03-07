use leptos::prelude::*;
use leptos_router::components::A;

/// Renders the home page of your application.
#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <div class="max-w-3xl mx-auto">
            <h1 class="text-4xl font-bold mb-6">"Léo Coletta"</h1>
            <h2 class="text-2xl font-semibold mb-4">"Software Engineer"</h2>

            <div class="prose max-w-none">
                <p class="mb-4">
                    "Welcome to my personal website! I'm a passionate software engineer specializing in Rust,
                    web technologies, and distributed systems."
                </p>

                <p class="mb-6">
                    "Check out my " <A href="/blog" attr:class="text-blue-600 hover:underline">
                        "blog"
                    </A>
                    " where I share technical tutorials, insights on programming, and my experiences in the tech industry."
                </p>
            </div>
        </div>
    }
}

