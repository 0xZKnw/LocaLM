pub mod conversation_list;
pub mod model_picker;

use crate::app::AppState;
use crate::storage::conversations::{list_conversations, save_conversation, Conversation};
use crate::ui::sidebar::conversation_list::ConversationList;
use crate::ui::sidebar::model_picker::ModelPicker;
use dioxus::prelude::*;

#[component]
pub fn Sidebar(on_settings_click: EventHandler<MouseEvent>, on_new_chat: EventHandler<()>) -> Element {
    let app_state = use_context::<AppState>();
    tracing::debug!("Sidebar rendered");

    let handle_new = {
        let mut conversations_signal = app_state.conversations.clone();
        let mut current_conversation_signal = app_state.current_conversation.clone();
        let on_new_chat = on_new_chat.clone();
        move |_| {
            tracing::info!("New Chat button clicked");
            let conversation = Conversation::new(None);
            if let Err(e) = save_conversation(&conversation) {
                tracing::error!("Failed to save conversation: {}", e);
                return;
            }
            current_conversation_signal.set(Some(conversation));
            if let Ok(conversations) = list_conversations() {
                conversations_signal.set(conversations);
            }
            on_new_chat.call(());
        }
    };
    
    rsx! {
        aside {
            class: "w-[280px] h-full flex flex-col bg-[var(--bg-sidebar)]/90 backdrop-blur-2xl backdrop-saturate-150 border-r border-[var(--border-subtle)] z-10 transition-colors duration-300",
            
            // Header with model picker
            div { 
                class: "p-4 border-b border-[var(--border-subtle)] space-y-4",
                
                // Model Selector
                ModelPicker {}

                // New Chat button
                button {
                    onclick: handle_new,
                    class: "w-full flex items-center gap-3 px-4 py-3 text-sm font-medium text-white bg-gradient-to-r from-[var(--accent-primary)] to-[var(--accent-hover)] rounded-xl shadow-lg shadow-purple-500/25 hover:shadow-purple-500/40 hover:-translate-y-0.5 transition-all duration-200 group",
                    
                    svg {
                        class: "w-4 h-4 transition-transform group-hover:rotate-90",
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2.5",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        path { d: "M12 5v14M5 12h14" }
                    }
                    "New Chat"
                }
            }
            
            // Conversation List
            // We wrap it in a div to match the structure, but ConversationList handles its own scrolling/container
            // We pass a class to ConversationList? No, it doesn't take props.
            // We'll trust ConversationList updates to match the style.
            ConversationList {}
            
            // Footer: User / Settings
            div {
                class: "p-4 border-t border-[var(--border-subtle)] bg-transparent", // Removed bg-[var(--bg-sidebar)] to keep glass effect
                
                // Settings button
                button {
                    onclick: on_settings_click,
                    class: "w-full flex items-center gap-3 px-4 py-3 text-sm text-[var(--text-secondary)] hover:text-[var(--text-primary)] rounded-xl hover:bg-white/[0.05] transition-all duration-150 group",
                    
                    div {
                        class: "p-1.5 rounded-md bg-[var(--bg-subtle)] text-[var(--text-tertiary)] group-hover:text-[var(--text-primary)] transition-colors",
                        svg {
                            class: "w-4 h-4 transition-transform group-hover:rotate-45",
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            circle { cx: "12", cy: "12", r: "3" }
                            path { d: "M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z" }
                        }
                    }
                    div {
                        class: "flex flex-col items-start",
                        span { class: "font-medium text-[var(--text-primary)]", "Settings" }
                        span { class: "text-xs text-[var(--text-tertiary)]", "Preferences" }
                    }
                }
            }
        }
    }
}
