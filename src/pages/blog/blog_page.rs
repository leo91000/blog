use crate::models::post::Post;
use crate::server::blog::get_posts;
use leptos::prelude::*;
use leptos_router::components::A;

/// Blog listing page
#[component]
pub fn BlogPage() -> impl IntoView {
    // Create resource to fetch posts
    let posts_resource = OnceResource::new(async move { get_posts(true).await });

    // Format date for display
    let format_date = |date: chrono::DateTime<chrono::Utc>| date.format("%B %d, %Y").to_string();

    view! {
        <div class="max-w-4xl mx-auto">
            <h1 class="text-3xl font-bold mb-8 bg-gradient-to-r from-primary-600 to-accent-600 inline-block text-transparent bg-clip-text">"Blog Posts"</h1>

            <Suspense fallback=|| {
                view! { 
                    <div class="animate-pulse flex space-x-4">
                        <div class="flex-1 space-y-6 py-1">
                            <div class="h-4 bg-gray-300 rounded w-3/4"></div>
                            <div class="space-y-3">
                                <div class="grid grid-cols-3 gap-4">
                                    <div class="h-4 bg-gray-300 rounded col-span-2"></div>
                                    <div class="h-4 bg-gray-300 rounded col-span-1"></div>
                                </div>
                                <div class="h-4 bg-gray-300 rounded"></div>
                            </div>
                        </div>
                    </div>
                }
            }>
                {move || match posts_resource.get() {
                    None => view! { <p>"Loading..."</p> }.into_any(),
                    Some(Err(e)) => {
                        view! {
                            <p class="text-red-500">"Error loading posts: " {e.to_string()}</p>
                        }
                            .into_any()
                    }
                    Some(Ok(posts)) => {
                        if posts.is_empty() {
                            view! { <p>"No posts available yet."</p> }.into_any()
                        } else {
                            view! {
                                <div class="space-y-8">
                                    <For
                                        each=move || posts.clone()
                                        key=|post| post.id
                                        children=move |post: Post| {
                                            view! {
                                                <article class="bg-white/80 backdrop-blur-sm p-6 rounded-xl shadow-lg hover:shadow-xl transition-all duration-300 border-l-4 border-primary-500 card">
                                                    <h2 class="text-2xl font-bold mb-2">
                                                        <A
                                                            href=format!("/blog/{}", post.id)
                                                            attr:class="text-primary-700 hover:text-primary-500 transition-colors duration-300"
                                                        >
                                                            {post.title.clone()}
                                                        </A>
                                                    </h2>
                                                    <div class="text-gray-500 mb-4 flex items-center text-sm">
                                                        <span class="inline-block w-2 h-2 rounded-full bg-accent-500 mr-2"></span>
                                                        {format_date(post.created_at)}
                                                    </div>
                                                    <div class="prose text-gray-700">
                                                        // Display a preview of the content
                                                        {post.content.chars().take(200).collect::<String>()}
                                                        {if post.content.len() > 200 { "..." } else { "" }}
                                                    </div>
                                                    <div class="mt-4">
                                                        <A
                                                            href=format!("/blog/{}", post.id)
                                                            attr:class="inline-flex items-center px-4 py-2 bg-primary-600 text-white rounded-lg hover:bg-primary-500 transition-colors duration-300 text-sm font-medium"
                                                        >
                                                            "Read more " <span class="ml-1">"→"</span>
                                                        </A>
                                                    </div>
                                                </article>
                                            }
                                        }
                                    />
                                </div>
                            }
                                .into_any()
                        }
                    }
                }}
            </Suspense>
        </div>
    }
}
