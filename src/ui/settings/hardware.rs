use crate::app::AppState;
use crate::storage::settings::save_settings;
use dioxus::prelude::*;

pub fn HardwareSettings() -> Element {
    let app_state = use_context::<AppState>();
    let settings = app_state.settings.read().clone();
    let gpu_layers = settings.gpu_layers;
    let models_dir = settings.models_directory.to_string_lossy().to_string();
    let mut app_state_gpu_layers = app_state.clone();

    // Mock Hardware Info
    let gpu_name = "NVIDIA GeForce RTX 4090 (Mock)";
    let vram_total_gb = 24.0;
    let vram_used_gb = 4.2;
    let vram_percent = (vram_used_gb / vram_total_gb) * 100.0;

    rsx! {
        div {
            class: "space-y-6 max-w-3xl mx-auto animate-fade-in pb-8",

            div {
                 class: "p-6 rounded-2xl bg-white/[0.03] backdrop-blur-md border border-white/[0.08]",

                h3 {
                    class: "text-xl font-semibold mb-6 text-[var(--text-primary)]",
                    "Hardware Acceleration"
                }

                 // GPU Info Card
                 div {
                     class: "p-6 rounded-xl mb-8 border relative overflow-hidden",
                     style: "background: linear-gradient(135deg, var(--bg-active) 0%, transparent 100%); border-color: var(--border-subtle);",

                    div { class: "flex items-start space-x-4 gap-4 relative z-10",
                        // Chip Icon
                        div {
                            class: "p-3 rounded-lg bg-[var(--accent-primary)]/10 text-[var(--accent-primary)]",
                            svg { class: "w-6 h-6", fill: "none", stroke: "currentColor", view_box: "0 0 24 24", stroke_width: "2",
                                path { d: "M9 3v2m6-2v2M9 19v2m6-2v2M5 9H3m2 6H3m18-6h-2m2 6h-2M7 19h10a2 2 0 002-2V7a2 2 0 00-2-2H7a2 2 0 00-2 2v10a2 2 0 002 2zM9 9h6v6H9V9z" }
                            }
                        }
                        div { class: "flex-1",
                            div { class: "font-bold text-lg text-[var(--text-primary)]", "{gpu_name}" }
                            div { class: "mt-4 space-y-2",
                                div { class: "flex justify-between text-sm text-[var(--text-secondary)]",
                                    span { "VRAM Usage" }
                                    span { class: "font-mono opacity-80", "{vram_used_gb} GB / {vram_total_gb} GB" }
                                }
                                // Progress Bar
                                div {
                                    class: "w-full rounded-full h-2.5 overflow-hidden bg-white/[0.1]",
                                    div {
                                        class: "h-2.5 rounded-full transition-all duration-500",
                                        style: "width: {vram_percent}%; background-color: var(--accent-primary);"
                                    }
                                }
                            }
                        }
                    }
                 }

                 // GPU Layers Control
                 div { class: "mb-8 space-y-3",
                    div { class: "flex justify-between items-center",
                        label { class: "font-medium text-[var(--text-primary)]", "GPU Layers" }
                        span {
                            class: "text-sm font-mono px-2 py-1 rounded bg-white/[0.05] text-[var(--text-secondary)] border border-white/[0.1]",
                            "{gpu_layers}"
                        }
                    }
                    input {
                        r#type: "range",
                        min: "0",
                        max: "99",
                        value: "{gpu_layers}",
                        oninput: move |e| {
                            let value = e.value().parse().unwrap_or(0);
                            let mut settings = app_state_gpu_layers.settings.write();
                            settings.gpu_layers = value;
                            if let Err(error) = save_settings(&settings) {
                                tracing::error!("Failed to save settings: {}", error);
                            }
                        },
                        class: "w-full h-2 rounded-lg appearance-none cursor-pointer bg-white/[0.1]",
                        style: "accent-color: var(--accent-primary);"
                    }
                    p { class: "text-xs text-[var(--text-secondary)] opacity-70",
                        "Number of model layers to offload to the GPU. Higher values improve inference speed but require more VRAM."
                    }
                 }

                 // Models Directory Input
                 div { class: "space-y-2",
                    label { class: "font-medium block text-[var(--text-primary)]", "Models Directory" }
                    div { class: "flex space-x-2 gap-2",
                        input {
                            r#type: "text",
                            readonly: true,
                            value: "{models_dir}",
                            class: "flex-1 p-3 rounded-lg border cursor-not-allowed bg-white/[0.05] border-white/[0.12] text-[var(--text-secondary)]",
                        }
                        button {
                            class: "px-4 py-2 border rounded-lg font-medium transition-colors shadow-sm bg-white/[0.05] border-white/[0.12] hover:bg-white/[0.1] text-[var(--text-primary)]",
                            "Browse..."
                        }
                    }
                    p { class: "text-xs text-[var(--text-secondary)] opacity-70",
                        "Location where model files (.gguf) are stored."
                    }
                 }
            }
        }
    }
}
