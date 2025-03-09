use crate::app::CurrentUser;
use crate::server::auth::logout;
use leptos::prelude::*;
use leptos::task::spawn_local;

#[component]
pub fn LogoutPage() -> impl IntoView {
    let navigate = leptos_router::hooks::use_navigate();
    let user_resource = expect_context::<CurrentUser>();

    spawn_local(async move {
        if (logout().await).is_ok() {
            user_resource.set(None);
            navigate("/", Default::default());
        }
    });

    view! {
        <div class="text-center py-8">
            <p>"Logging out..."</p>
        </div>
    }
}
