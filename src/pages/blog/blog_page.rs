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
        <div class="max-w-4xl mx-auto w-full">
            <h1 class="text-3xl font-bold mb-8 bg-gradient-to-r from-primary-600 to-accent-600 text-transparent bg-clip-text flex items-center gap-2 flex items-center justify-center gap-2">
                <span class="i-mdi-post text-primary-500 dark:text-primary-400"></span>
                "Blog Posts"
            </h1>

            <Suspense fallback=move || {
                view! { <p>"Loading..."</p> }
            }>
                {move || Suspend::new(async move {
                    posts_resource.await.ok();
                    {
                        move || {
                            posts_resource
                                .get()
                                .map(|posts| match posts {
                                    Err(e) => {
                                        view! {
                                            <div class="bg-red-100 dark:bg-red-900/30 border border-red-300 dark:border-red-700 text-red-700 dark:text-red-300 px-4 py-3 rounded-lg mb-6 flex items-center gap-2">
                                                <span class="i-mdi-alert-circle text-lg"></span>
                                                <p>"Error loading posts: " {e.to_string()}</p>
                                            </div>
                                        }
                                            .into_any()
                                    }
                                    Ok(posts) => {
                                        if posts.is_empty() {
                                            view! {
                                                <div class="text-center py-12 bg-white/90 dark:bg-primary-800/90 p-8 rounded-xl shadow-lg dark:shadow-primary-900/50 backdrop-blur-sm border border-gray-100 dark:border-primary-700">
                                                    <div class="inline-flex p-3 bg-blue-100 dark:bg-blue-900/30 rounded-full mb-6 text-blue-500 dark:text-blue-400">
                                                        <span class="i-mdi-information text-3xl"></span>
                                                    </div>
                                                    <h2 class="text-xl font-bold mb-4 dark:text-white">
                                                        "No Posts Available"
                                                    </h2>
                                                    <p class="mb-6 dark:text-gray-300">
                                                        "There are no blog posts available yet. Check back later!"
                                                    </p>
                                                </div>
                                            }
                                                .into_any()
                                        } else {
                                            view! {
                                                <div class="space-y-8">
                                                    <For
                                                        each=move || posts.clone()
                                                        key=|post| post.id
                                                        children=move |post: Post| {
                                                            view! {
                                                                <article class="bg-white/80 dark:bg-primary-800/80 backdrop-blur-sm p-6 rounded-xl shadow-lg hover:shadow-xl transition-all duration-300 border-l-4 border-primary-500 dark:border-primary-400 card">
                                                                    <h2 class="text-2xl font-bold mb-2">
                                                                        <A
                                                                            href=format!("/blog/{}", post.id)
                                                                            attr:class="text-primary-700 dark:text-primary-400 hover:text-primary-500 dark:hover:text-primary-300 transition-colors duration-300"
                                                                        >
                                                                            {post.title.clone()}
                                                                        </A>
                                                                    </h2>
                                                                    <div class="text-gray-500 dark:text-gray-300 mb-4 flex items-center text-sm">
                                                                        <span class="inline-block w-2 h-2 rounded-full bg-accent-500 mr-2"></span>
                                                                        {format_date(post.created_at)}
                                                                    </div>
                                                                    <div class="prose dark:prose-invert text-gray-700 dark:text-gray-200">
                                                                        // Display a preview of the content
                                                                        {post.content.chars().take(200).collect::<String>()}
                                                                        {if post.content.len() > 200 { "..." } else { "" }}
                                                                    </div>
                                                                    <div class="mt-4">
                                                                        <A
                                                                            href=format!("/blog/{}", post.id)
                                                                            attr:class="inline-flex items-center px-4 py-2 bg-gradient-to-r from-primary-500 to-accent-500 text-white rounded-lg hover:from-primary-600 hover:to-accent-600 transition-all duration-300 text-sm font-medium gap-1"
                                                                        >
                                                                            "Read more"
                                                                            <span class="i-mdi-arrow-right"></span>
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
                                })
                        }
                    }
                })}
            </Suspense>
        </div>
    }
}
