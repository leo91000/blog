use leptos::prelude::*;
use leptos_router::components::A;

/// Renders the home page of your application.
#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <div class="max-w-3xl mx-auto text-center">
            <div class="mb-8">
                <h1 class="text-5xl font-bold mb-4 bg-gradient-to-r from-primary-600 to-accent-600 text-transparent bg-clip-text">
                    "Léo Coletta"
                </h1>
                <h2 class="text-2xl font-semibold text-gray-700 dark:text-gray-200 mb-4">
                    "Software Engineer"
                </h2>
            </div>

            <div class="bg-white/50 dark:bg-primary-700/50 backdrop-blur-sm p-8 rounded-xl shadow-lg card">
                <p class="text-lg mb-6 text-gray-800 dark:text-gray-100">
                    "Welcome to my personal website! I'm a passionate software engineer specializing in Rust,
                    web technologies, and distributed systems."
                </p>

                <div class="grid grid-cols-1 md:grid-cols-2 gap-6 mt-8">
                    <A
                        href="/blog"
                        attr:class="bg-gradient-to-br from-primary-500 to-primary-600 hover:from-primary-400 hover:to-primary-500 p-6 rounded-xl text-white shadow-md transition-all duration-300 hover-grow text-center"
                    >
                        <div class="text-xl font-bold mb-2">"Read My Blog"</div>
                        <p class="text-white/90 text-sm">
                            "Technical tutorials and insights on programming"
                        </p>
                    </A>

                    <div class="bg-gradient-to-br from-accent-500 to-accent-600 p-6 rounded-xl text-white shadow-md transition-all duration-300 hover-grow text-center">
                        <div class="text-xl font-bold mb-2">"Contact Me"</div>
                        <p class="text-white/90 text-sm">
                            "Get in touch for professional opportunities"
                        </p>
                    </div>
                </div>
            </div>
        </div>
    }
}

