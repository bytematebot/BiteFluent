use crate::api::projects::ProjectWorkspaceKeyDto;
use crate::components::{IconKind, RenderIcon};
use dioxus::prelude::*;

#[component]
pub fn ProjectKeysPanel(
    keys: Vec<ProjectWorkspaceKeyDto>,
    selected_key: Option<String>,
    on_select_key: EventHandler<String>,
) -> Element {
    let keys_count_label = match keys.len() {
        1 => "1 key".to_string(),
        count => format!("{count} keys"),
    };

    rsx! {
        section {
            class: "flex h-full min-h-0 flex-col overflow-hidden border-r border-white/[0.08] bg-white/[0.015]",

            div {
                class: "shrink-0 border-b border-white/[0.08] p-5",

                p {
                    class: "text-sm font-semibold text-[color:var(--text-primary)]",
                    "Translation keys"
                }

                p {
                    class: "mt-1 text-xs text-[color:var(--text-muted)]",
                    "{keys_count_label}"
                }
            }

            div {
                class: "min-h-0 flex-1 overflow-y-auto p-3 [scrollbar-color:rgba(255,255,255,0.16)_transparent] [scrollbar-width:thin]",

                if keys.is_empty() {
                    p {
                        class: "rounded-xl border border-white/[0.08] bg-white/[0.02] p-4 text-sm leading-6 text-[color:var(--text-muted)]",
                        "Select a file with Fluent messages."
                    }
                } else {
                    div {
                        class: "space-y-1 pb-3",

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
        "group w-full rounded-xl border border-white/[0.10] bg-white/[0.055] px-3 py-3 text-left"
    } else {
        "group w-full rounded-xl border border-transparent px-3 py-3 text-left transition hover:border-white/[0.08] hover:bg-white/[0.035]"
    };

    let icon_class = if selected {
        "h-3.5 w-3.5 shrink-0 text-[color:var(--text-secondary)]"
    } else {
        "h-3.5 w-3.5 shrink-0 text-[color:var(--text-muted)] transition group-hover:text-[color:var(--text-secondary)]"
    };

    let key_class = if selected {
        "truncate text-sm font-semibold text-[color:var(--text-primary)]"
    } else {
        "truncate text-sm font-medium text-[color:var(--text-primary)]"
    };

    rsx! {
        button {
            class: "{class}",
            type: "button",
            title: "{entry.key}",
            onclick: {
                let key = entry.key.clone();

                move |_| {
                    on_select.call(key.clone());
                }
            },

            div {
                class: "flex min-w-0 items-center gap-2",

                RenderIcon {
                    kind: IconKind::Code,
                    class: Some(icon_class.to_string()),
                }

                p {
                    class: "{key_class}",
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
