//! UI components for LocaLM
//!
//! This module contains all user interface components built with Dioxus.

pub mod chat;
pub mod components;
pub mod settings;
pub mod sidebar;

use crate::ui::sidebar::Sidebar;
use crate::ui::chat::ChatView;
use crate::ui::settings::Settings as SettingsPanel;
use crate::app::AppState;
use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq)]
enum MainView {
    Chat,
    Settings,
}

/// Main Application Layout
#[component]
pub fn Layout() -> Element {
    let mut current_view = use_signal(|| MainView::Chat);
    let app_state = use_context::<AppState>();
    
    // Get theme from settings
    let theme_str = app_state.settings.read().theme.clone();

    rsx! {
        // Theme wrapper
        div {
            "data-theme": "{theme_str}",
            class: "flex h-screen w-screen bg-[var(--bg-main)] text-[var(--text-primary)] transition-colors duration-300 overflow-hidden",

            // Inline CSS using include_str! macro for desktop app compatibility
            style { {include_str!("../../assets/styles.css")} }

            // Sidebar
            Sidebar {
                on_settings_click: move |_| current_view.set(MainView::Settings),
                on_new_chat: move |_| current_view.set(MainView::Chat)
            }

            // Main Content Area
            main {
                class: "flex-1 flex flex-col h-full relative min-w-0 bg-[var(--bg-main)] transition-colors duration-300",

                // Main Content (ChatView, Settings, or Welcome Screen)
                if current_view() == MainView::Settings {
                    div {
                        class: "flex flex-col h-full",
                        // Back Button Header
                        div {
                            class: "flex-none px-8 pt-6 pb-2",
                            button {
                                onclick: move |_| current_view.set(MainView::Chat),
                                class: "flex items-center gap-2 text-[var(--text-secondary)] hover:text-[var(--text-primary)] transition-colors text-sm font-medium group",
                                svg {
                                    class: "w-4 h-4 transition-transform group-hover:-translate-x-1",
                                    view_box: "0 0 24 24",
                                    fill: "none",
                                    stroke: "currentColor",
                                    stroke_width: "2",
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    path { d: "M19 12H5M12 19l-7-7 7-7" }
                                }
                                "Back to Chat"
                            }
                        }
                        SettingsPanel {}
                    }
                } else if app_state.current_conversation.read().is_some() {
                    ChatView {}
                } else {
                    // Welcome Screen - Modern Glassmorphism
                    div {
                        class: "flex-1 flex flex-col items-center justify-center p-8 text-center",
                        
                        // Logo with enhanced glow
                        div {
                            class: "mb-12 relative",
                            div { 
                                class: "absolute inset-0 bg-[var(--accent-primary)] blur-[80px] opacity-30 animate-pulse" 
                            }
                            div {
                                class: "relative w-28 h-28 bg-gradient-to-br from-[var(--accent-primary)] to-[var(--accent-secondary)] rounded-3xl flex items-center justify-center text-white shadow-2xl shadow-purple-500/30",
                                svg { 
                                    width: "48", 
                                    height: "48", 
                                    view_box: "0 0 24 24", 
                                    fill: "none", 
                                    stroke: "currentColor", 
                                    stroke_width: "2", 
                                    path { d: "M21 15a2 2 0 0 1-2 2H7l-4 4V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2z" } 
                                }
                            }
                        }
                        
                        // Title with gradient text
                        h1 {
                            class: "text-5xl font-bold mb-4 bg-gradient-to-r from-[var(--accent-primary)] to-[var(--accent-secondary)] bg-clip-text text-transparent",
                            "LocaLM"
                        }
                        p { 
                            class: "text-lg text-[var(--text-secondary)] max-w-md mx-auto leading-relaxed mb-12", 
                            "Your private AI companion. Fast, local, and secure." 
                        }
                        
                        // Glass cards for actions
                        div {
                            class: "grid grid-cols-1 md:grid-cols-2 gap-4 w-full max-w-lg mt-8",
                            
                            // New Chat card
                            button {
                                onclick: move |_| current_view.set(MainView::Chat),
                                class: "flex items-center gap-4 p-6 rounded-2xl bg-white/[0.03] backdrop-blur-md border border-white/[0.08] hover:bg-white/[0.06] hover:border-white/[0.12] hover:-translate-y-1 transition-all duration-300 group text-left",
                                div {
                                    class: "p-3 bg-white/[0.05] rounded-xl text-[var(--accent-primary)] group-hover:bg-[var(--accent-primary)] group-hover:text-white transition-colors duration-300",
                                    svg { 
                                        width: "24", 
                                        height: "24", 
                                        view_box: "0 0 24 24", 
                                        fill: "none", 
                                        stroke: "currentColor", 
                                        stroke_width: "2", 
                                        path { d: "M12 5v14M5 12h14" } 
                                    }
                                }
                                div {
                                    div { class: "font-semibold text-[var(--text-primary)] mb-0.5", "New Conversation" }
                                    div { class: "text-sm text-[var(--text-secondary)] opacity-70", "Start a fresh chat" }
                                }
                            }
                            
                            // Settings card
                            button {
                                onclick: move |_| current_view.set(MainView::Settings),
                                class: "flex items-center gap-4 p-6 rounded-2xl bg-white/[0.03] backdrop-blur-md border border-white/[0.08] hover:bg-white/[0.06] hover:border-white/[0.12] hover:-translate-y-1 transition-all duration-300 group text-left",
                                div {
                                    class: "p-3 bg-white/[0.05] rounded-xl text-[var(--text-secondary)] group-hover:bg-[var(--text-primary)] group-hover:text-[var(--bg-main)] transition-colors duration-300",
                                    svg { 
                                        width: "24", 
                                        height: "24", 
                                        view_box: "0 0 24 24", 
                                        fill: "none", 
                                        stroke: "currentColor", 
                                        stroke_width: "2", 
                                        circle { cx: "12", cy: "12", r: "3" }, 
                                        path { d: "M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z" } 
                                    }
                                }
                                div {
                                    div { class: "font-semibold text-[var(--text-primary)] mb-0.5", "Settings" }
                                    div { class: "text-sm text-[var(--text-secondary)] opacity-70", "Configure models" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
