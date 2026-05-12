use crate::api::projects::{ProjectDto, ProjectWorkspaceFileDto};
use crate::components::{IconKind, RenderIcon};
use dioxus::prelude::*;

#[component]
pub fn ProjectSidebar(
    project: ProjectDto,
    files: Vec<ProjectWorkspaceFileDto>,
    selected_file: Option<String>,
    on_select_file: EventHandler<String>,
) -> Element {
    rsx! {
        aside {
            class: "border-r border-white/[0.08] bg-black/[0.10]",

            div {
                class: "border-b border-white/[0.08] p-5",

                p {
                    class: "truncate text-sm font-semibold text-[color:var(--text-primary)]",
                    "{project.name}"
                }

                p {
                    class: "mt-1 truncate text-xs text-[color:var(--text-muted)]",
                    "{project.repository_full_name}"
                }
            }

            div {
                class: "p-3",

                p {
                    class: "mb-2 px-2 text-xs font-semibold uppercase tracking-[0.18em] text-[color:var(--text-muted)]",
                    "Files"
                }

                if files.is_empty() {
                    p {
                        class: "rounded-xl border border-white/[0.08] bg-white/[0.02] p-4 text-sm leading-6 text-[color:var(--text-muted)]",
                        "No Fluent files found."
                    }
                } else {
                    div {
                        class: "space-y-1",

                        for file in files {
                            FileButton {
                                file: file.clone(),
                                selected: selected_file
                                    .as_ref()
                                    .map(|path| path == &file.path)
                                    .unwrap_or(false),
                                on_select: move |path| on_select_file.call(path),
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn FileButton(
    file: ProjectWorkspaceFileDto,
    selected: bool,
    on_select: EventHandler<String>,
) -> Element {
    let class = if selected {
        "flex w-full items-center gap-3 rounded-xl border border-[color:var(--bg-accent)] bg-[var(--bg-accent)]/[0.08] px-3 py-2.5 text-left"
    } else {
        "flex w-full items-center gap-3 rounded-xl border border-transparent px-3 py-2.5 text-left transition hover:border-white/[0.08] hover:bg-white/[0.035]"
    };

    rsx! {
        button {
            class: "{class}",
            type: "button",
            onclick: {
                let path = file.path.clone();

                move |_| {
                    on_select.call(path.clone());
                }
            },

            RenderIcon {
                kind: IconKind::FileText,
                class: Some("size-4 shrink-0 text-[color:var(--text-muted)]".to_string()),
            }

            span {
                class: "min-w-0 flex-1",

                span {
                    class: "block truncate text-sm font-medium text-[color:var(--text-primary)]",
                    "{file.name}"
                }

                if let Some(locale) = file.locale.as_ref() {
                    span {
                        class: "mt-0.5 block truncate text-xs text-[color:var(--text-muted)]",
                        "{locale}"
                    }
                }
            }
        }
    }
}