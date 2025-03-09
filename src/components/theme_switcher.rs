use crate::models::session::ThemePreference;
use crate::server::session::{
    get_theme_preference, set_theme_preference, GetThemePreferenceResponse,
};
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_meta::Html;

// Define a shared theme context type for use across components
#[derive(Copy, Clone, Debug)]
pub struct ThemeContext {
    pub current_theme: ReadSignal<ThemePreference>,
    pub set_theme: WriteSignal<ThemePreference>,
}

#[component]
fn ThemeProviderInner(
    children: Children,
    server_theme: GetThemePreferenceResponse,
) -> impl IntoView {
    let (current_theme, set_current_theme) = signal(server_theme.theme_preference);
    let theme_ctx = ThemeContext {
        current_theme,
        set_theme: set_current_theme,
    };
    provide_context(theme_ctx);
    let dark_class = move || {
        let prefer_dark = server_theme
            .theme_preference_header
            .is_some_and(|t| t == ThemePreference::Dark);
        current_theme.get() == ThemePreference::Dark
            || (prefer_dark && current_theme.get() == ThemePreference::System)
    };
    view! {
        {children()}
        <Html {..} class:dark=dark_class />
    }
}

#[component]
pub fn ThemeProvider(children: Children) -> impl IntoView {
    let theme_resource =
        OnceResource::new(async { get_theme_preference().await.unwrap_or_default() });

    view! {
        <Await future=theme_resource.into_future() let:server_theme>
            <ThemeProviderInner server_theme=server_theme.clone()>{children()}</ThemeProviderInner>
        </Await>
    }
}

// ThemeSwitcher component that uses the shared context
#[component]
pub fn ThemeSwitcher() -> impl IntoView {
    // Try to use the existing theme context
    let theme_ctx = expect_context::<ThemeContext>();

    // If no context is available, create a local theme state
    let current_theme = theme_ctx.current_theme;
    let set_current_theme = theme_ctx.set_theme;

    let toggle_theme = move |_| {
        let new_theme = match current_theme.get() {
            ThemePreference::System => ThemePreference::Light,
            ThemePreference::Light => ThemePreference::Dark,
            ThemePreference::Dark => ThemePreference::System,
        };
        set_current_theme.set(new_theme);
        spawn_local(async move {
            let _ = set_theme_preference(new_theme).await;
        });
    };

    let theme_text = move || match current_theme.get() {
        ThemePreference::System => "System",
        ThemePreference::Light => "Light",
        ThemePreference::Dark => "Dark",
    };

    let theme_icon = move || match current_theme.get() {
        ThemePreference::System => "i-mdi-monitor",
        ThemePreference::Light => "i-mdi-white-balance-sunny",
        ThemePreference::Dark => "i-mdi-moon-waning-crescent",
    };

    view! {
        <button
            on:click=toggle_theme
            class="p-2 rounded-full bg-white/20 dark:bg-primary-700 hover:bg-white/30 dark:hover:bg-primary-600 transition-all duration-300 flex items-center space-x-2 hover:cursor-pointer"
            title=move || format!("Current theme: {}", theme_text())
        >
            <span class=move || format!("{} text-lg", theme_icon())></span>
        </button>
    }
}
