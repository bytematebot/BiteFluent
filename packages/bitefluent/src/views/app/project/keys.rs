use crate::api::projects::ProjectWorkspaceKeyDto;
use crate::components::{IconKind, RenderIcon};
use dioxus::prelude::*;

#[component]
pub fn ProjectKeysPanel(
    keys: Vec<ProjectWorkspaceKeyDto>,
    selected_key: Option<String>,
    on_select_key: EventHandler<String>,
) -> Element {
    rsx! {
        section {
            class: "border-r border-white/[0.08] bg-white/[0.015]",

            div {
                class: "border-b border-white/[0.08] p-5",

                p {
                    class: "text-sm font-semibold text-[color:var(--text-primary)]",
                    "Translation keys"
                }

                p {
                    class: "mt-1 text-xs text-[color:var(--text-muted)]",
                    "{keys.len()} keys"
                }
            }

            div {
                class: "p-3",

                if keys.is_empty() {
                    p {
                        class: "rounded-xl border border-white/[0.08] bg-white/[0.02] p-4 text-sm leading-6 text-[color:var(--text-muted)]",
                        "Select a file with Fluent messages."
                    }
                } else {
                    div {
                        class: "space-y-1",

                        for key in keys {
                            KeyButton {
                                entry: key.clone(),
                                selected: selected_key
                                    .as_ref()
                                    .map(|selected| selected == &key.key)
                                    .unwrap_or(false),
                                on_select: move |key| on_select_key.call(key),
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn KeyButton(
    entry: ProjectWorkspaceKeyDto,
    selected: bool,
    on_select: EventHandler<String>,
) -> Element {
    let class = if selected {
        "w-full rounded-xl border border-[color:var(--bg-accent)] bg-[var(--bg-accent)]/[0.08] px-3 py-3 text-left"
    } else {
        "w-full rounded-xl border border-transparent px-3 py-3 text-left transition hover:border-white/[0.08] hover:bg-white/[0.035]"
    };

    rsx! {
        button {
            class: "{class}",
            type: "button",
            onclick: {
                let key = entry.key.clone();

                move |_| {
                    on_select.call(key.clone());
                }
            },

            div {
                class: "flex items-center gap-2",

                RenderIcon {
                    kind: IconKind::Code,
                    class: Some("size-3.5 shrink-0 text-[color:var(--text-muted)]".to_string()),
                }

                p {
                    class: "truncate text-sm font-semibold text-[color:var(--text-primary)]",
                    "{entry.key}"
                }
            }

            p {
                class: "mt-1 line-clamp-2 text-xs leading-5 text-[color:var(--text-muted)]",
                "{entry.value}"
            }
        }
    }
}