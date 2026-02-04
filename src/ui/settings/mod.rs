#![allow(non_snake_case)]

pub mod appearance;
pub mod hardware;
pub mod inference;

use crate::ui::settings::appearance::AppearanceSettings;
use crate::ui::settings::hardware::HardwareSettings;
use crate::ui::settings::inference::InferenceSettings;
use dioxus::prelude::*;

#[derive(PartialEq, Clone, Copy)]
enum SettingsTab {
    Inference,
    Hardware,
    Appearance,
}

pub fn Settings() -> Element {
    let mut active_tab = use_signal(|| SettingsTab::Inference);

    rsx! {
        div {
            class: "flex flex-col h-full bg-[var(--bg-main)]",

            // Header with glass effect
            div {
                class: "flex-none px-8 py-6 border-b border-[var(--border-subtle)] bg-[var(--bg-secondary)]/50 backdrop-blur-xl",

                div {
                    class: "max-w-3xl mx-auto w-full",

                    h2 {
                        class: "text-2xl font-bold tracking-tight mb-6 text-[var(--text-primary)]",
                        "Settings"
                    }

                    // Tabs
                    div {
                        class: "flex gap-2 p-1 bg-white/[0.03] rounded-xl",

                        TabButton {
                            active: active_tab() == SettingsTab::Inference,
                            onclick: move |_| active_tab.set(SettingsTab::Inference),
                            label: "Inference",
                            icon: rsx! {
                                svg { class: "w-5 h-5 mr-2", fill: "none", stroke: "currentColor", view_box: "0 0 24 24", stroke_width: "2",
                                    path { stroke_linecap: "round", stroke_linejoin: "round", d: "M13 10V3L4 14h7v7l9-11h-7z" }
                                }
                            }
                        }
                        TabButton {
                            active: active_tab() == SettingsTab::Hardware,
                            onclick: move |_| active_tab.set(SettingsTab::Hardware),
                            label: "Hardware",
                            icon: rsx! {
                                svg { class: "w-5 h-5 mr-2", fill: "none", stroke: "currentColor", view_box: "0 0 24 24", stroke_width: "2",
                                    path { stroke_linecap: "round", stroke_linejoin: "round", d: "M9 3v2m6-2v2M9 19v2m6-2v2M5 9H3m2 6H3m18-6h-2m2 6h-2M7 19h10a2 2 0 002-2V7a2 2 0 00-2-2H7a2 2 0 00-2 2v10a2 2 0 002 2zM9 9h6v6H9V9z" }
                                }
                            }
                        }
                        TabButton {
                            active: active_tab() == SettingsTab::Appearance,
                            onclick: move |_| active_tab.set(SettingsTab::Appearance),
                            label: "Appearance",
                            icon: rsx! {
                                svg { class: "w-5 h-5 mr-2", fill: "none", stroke: "currentColor", view_box: "0 0 24 24", stroke_width: "2",
                                    path { stroke_linecap: "round", stroke_linejoin: "round", d: "M7 21a4 4 0 01-4-4V5a2 2 0 012-2h4a2 2 0 012-2h4a2 2 0 012 2v12a4 4 0 01-4 4zm0 0h12a2 2 0 002-2v-4a2 2 0 00-2-2h-2.343M11 7.343l1.657-1.657a2 2 0 012.828 0l2.829 2.829a2 2 0 010 2.828l-8.486 8.485M7 17h.01" }
                                }
                            }
                        }
                    }
                }
            }

            // Content Area
            div {
                class: "flex-1 overflow-y-auto p-8",
                match active_tab() {
                    SettingsTab::Inference => rsx! { InferenceSettings {} },
                    SettingsTab::Hardware => rsx! { HardwareSettings {} },
                    SettingsTab::Appearance => rsx! { AppearanceSettings {} },
                }
            }
        }
    }
}

#[component]
fn TabButton(
    active: bool,
    onclick: EventHandler<MouseEvent>,
    label: String,
    icon: Element,
) -> Element {
    let classes = if active {
        "bg-white/[0.08] text-[var(--accent-primary)] shadow-sm"
    } else {
        "text-[var(--text-secondary)] hover:text-[var(--text-primary)] hover:bg-white/[0.04]"
    };

    rsx! {
        button {
            class: "flex-1 flex items-center justify-center py-2.5 px-4 rounded-lg text-sm font-medium transition-all duration-200 {classes}",
            onclick: onclick,
            {icon}
            "{label}"
        }
    }
}
