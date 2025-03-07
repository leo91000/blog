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
            <h1 class="text-3xl font-bold mb-8">"Blog Posts"</h1>

            {move || match posts_resource.get() {
                None => view! { <p>"Loading..."</p> }.into_any(),
                Some(Err(e)) => {
                    view! { <p class="text-red-500">"Error loading posts: " {e.to_string()}</p> }
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
                                            <article class="bg-white p-6 rounded shadow">
                                                <h2 class="text-2xl font-bold mb-2">
                                                    <A
                                                        href=format!("/blog/{}", post.id)
                                                        attr:class="hover:text-blue-600"
                                                    >
                                                        {post.title.clone()}
                                                    </A>
                                                </h2>
                                                <div class="text-gray-500 mb-4">
                                                    {format_date(post.created_at)}
                                                </div>
                                                <div class="prose">
                                                    // Display a preview of the content
                                                    {post.content.chars().take(200).collect::<String>()}
                                                    {if post.content.len() > 200 { "..." } else { "" }}
                                                </div>
                                                <div class="mt-4">
                                                    <A
                                                        href=format!("/blog/{}", post.id)
                                                        attr:class="text-blue-600 hover:underline"
                                                    >
                                                        "Read more →"
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
        </div>
    }
}

