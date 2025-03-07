use crate::server::auth::logout;
use leptos::prelude::*;
use leptos::task::spawn_local;

/// Logout page component
#[component]
pub fn LogoutPage() -> impl IntoView {
    // Navigate after successful logout
    let navigate = leptos_router::hooks::use_navigate();

    // Execute logout server function directly
    spawn_local(async move {
        if (logout().await).is_ok() {
            // Redirect to home
            navigate("/", Default::default());
        }
    });

    view! {
        <div class="text-center py-8">
            <p>"Logging out..."</p>
        </div>
    }
}