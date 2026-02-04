use crate::app::AppState;
use crate::storage::settings::save_settings;
use dioxus::prelude::*;

pub fn AppearanceSettings() -> Element {
    let app_state = use_context::<AppState>();
    let settings = app_state.settings.read().clone();
    let dark_mode = settings.theme == "dark";
    let font_size = settings.font_size.to_lowercase();
    let selected_font_size = match font_size.as_str() {
        "small" => "Small",
        "large" => "Large",
        _ => "Medium",
    };
    let mut app_state_theme = app_state.clone();
    let mut app_state_font_size = app_state.clone();

    rsx! {
        div {
            class: "space-y-6 max-w-3xl mx-auto animate-fade-in pb-8",

            // Card 1: Theme
            div {
                class: "p-6 rounded-2xl bg-white/[0.03] backdrop-blur-md border border-white/[0.08]",

                h3 {
                    class: "text-xl font-semibold mb-6 text-[var(--text-primary)]",
                    "Interface Appearance"
                }

                // Theme Toggle
                div {
                    class: "flex items-center justify-between py-2",

                    div {
                        div { class: "font-medium text-[var(--text-primary)]", "Dark Mode" }
                        div { class: "text-sm text-[var(--text-secondary)] opacity-80", "Toggle application color theme" }
                    }
                    button {
                        onclick: move |_| {
                            let mut settings = app_state_theme.settings.write();
                            settings.theme = if dark_mode { "light".to_string() } else { "dark".to_string() };
                            if let Err(error) = save_settings(&settings) {
                                tracing::error!("Failed to save settings: {}", error);
                            }
                        },
                        class: "relative inline-flex h-7 w-12 items-center rounded-full transition-colors focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-[var(--accent-primary)]",
                        style: format!(
                            "background-color: {};",
                            if dark_mode { "var(--accent-primary)" } else { "rgba(255, 255, 255, 0.1)" }
                        ),
                        span {
                            class: "inline-block h-5 w-5 transform rounded-full bg-white transition-transform shadow-sm",
                            style: format!(
                                "transform: translateX({});",
                                if dark_mode { "1.5rem" } else { "0.25rem" }
                            )
                        }
                    }
                }
            }

            // Card 2: Font Size
             div {
                class: "p-6 rounded-2xl bg-white/[0.03] backdrop-blur-md border border-white/[0.08]",

                div { class: "space-y-4",
                    div {
                        div { class: "font-medium text-[var(--text-primary)]", "Font Size" }
                        div { class: "text-sm text-[var(--text-secondary)] opacity-80", "Adjust the text size of the chat interface" }
                    }
                    div { class: "grid grid-cols-3 gap-4",
                        for size in &["Small", "Medium", "Large"] {
                            button {
                                onclick: move |_| {
                                    let mut settings = app_state_font_size.settings.write();
                                    settings.font_size = size.to_lowercase();
                                    if let Err(error) = save_settings(&settings) {
                                        tracing::error!("Failed to save settings: {}", error);
                                    }
                                },
                                class: format!(
                                    "p-4 rounded-xl border transition-all duration-200 {}",
                                    if selected_font_size == *size {
                                        "border-[var(--accent-primary)] bg-[var(--accent-primary)]/10 text-[var(--accent-primary)] shadow-sm"
                                    } else {
                                        "border-white/[0.08] bg-white/[0.02] text-[var(--text-secondary)] hover:bg-white/[0.05] hover:border-white/[0.12]"
                                    }
                                ),
                                div { class: "font-semibold mb-1", "{size}" }
                                div {
                                    class: "opacity-60",
                                    style: match *size {
                                        "Small" => "font-size: 0.875rem;",
                                        "Medium" => "font-size: 1rem;",
                                        "Large" => "font-size: 1.25rem;",
                                        _ => ""
                                    },
                                    "Aa"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
