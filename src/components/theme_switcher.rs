use crate::models::session::ThemePreference;
use crate::server::session::{get_theme_preference, set_theme_preference};
use leptos::logging::debug_warn;
use leptos::prelude::*;
use leptos::task::spawn_local;

pub type ColorThemeSignalContext = (ReadSignal<ThemePreference>, WriteSignal<ThemePreference>);

pub fn use_color_theme() -> ColorThemeSignalContext {
    let current_theme_signal = use_context::<ColorThemeSignalContext>();
    if let Some(current_theme_signal) = current_theme_signal {
        return current_theme_signal;
    }

    let current_theme_signal = signal(ThemePreference::System);
    provide_context(current_theme_signal);

    current_theme_signal
}

pub type ThemeServerResourceContext = OnceResource<Result<ThemePreference, ServerFnError>>;
pub fn use_theme_server_resource() -> ThemeServerResourceContext {
    let current_theme_server_resource = use_context::<ThemeServerResourceContext>();
    if let Some(current_theme_server_resource) = current_theme_server_resource {
        return current_theme_server_resource;
    }

    let current_theme_server_resource = OnceResource::new(async { get_theme_preference().await });
    provide_context(current_theme_server_resource);

    current_theme_server_resource
}

#[component]
pub fn ThemeSwitcher() -> impl IntoView {
    let theme_resource = use_theme_server_resource();
    let (current_theme, set_current_theme) = use_color_theme();

    // Update the theme when resource loads
    Effect::new(move || {
        if let Some(Ok(theme)) = theme_resource.get() {
            set_current_theme.set(theme);
        }
    });

    // Handle theme toggle
    let toggle_theme = move |_| {
        let new_theme = match current_theme.get() {
            ThemePreference::System => ThemePreference::Light,
            ThemePreference::Light => ThemePreference::Dark,
            ThemePreference::Dark => ThemePreference::System,
        };

        leptos::logging::log!("new_theme: {:?}", new_theme);
        set_current_theme.set(new_theme);

        // Call server function to update preference
        spawn_local(async move {
            let _ = set_theme_preference(new_theme).await;
        });
    };

    Effect::new(move || {
        let theme_class = match current_theme.get() {
            ThemePreference::Light => Some("light"),
            ThemePreference::Dark => Some("dark"),
            ThemePreference::System => None,
        };

        debug_warn!(
            "theme_class: {:?}, todo: add class to html root",
            theme_class
        );
        // How to add class to html root here ?
    });

    let theme_text = move || match current_theme.get() {
        ThemePreference::System => "System",
        ThemePreference::Light => "Light",
        ThemePreference::Dark => "Dark",
    };

    let theme_icon = move || match current_theme.get() {
        ThemePreference::System => "🖥️",
        ThemePreference::Light => "☀️",
        ThemePreference::Dark => "🌙",
    };

    view! {
        <Transition fallback=|| {
            view! { "Loading theme..." }
        }>
            {move || Suspend::new(async move {
                let theme = theme_resource.await;
                if let Ok(theme) = theme {
                    set_current_theme.set(theme);
                }

                view! {
                    <button
                        on:click=toggle_theme
                        class="px-3 py-1 rounded-full bg-white/20 hover:bg-white/30 transition-all duration-300 flex items-center space-x-1"
                        title=move || format!("Current theme: {}", theme_text())
                    >
                        <span>{theme_icon}</span>
                        <span class="text-sm">{theme_text}</span>
                    </button>
                }
            })}
        </Transition>
    }
}
