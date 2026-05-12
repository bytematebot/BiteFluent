use crate::api::projects::{ProjectDto, ProjectWorkspaceKeyDto};
use dioxus::prelude::*;

#[component]
pub fn ProjectEditorPanel(
    project: ProjectDto,
    entry: Option<ProjectWorkspaceKeyDto>,
) -> Element {
    rsx! {
        section {
            class: "flex min-w-0 flex-col bg-black/[0.08]",

            div {
                class: "border-b border-white/[0.08] p-5",

                p {
                    class: "text-sm font-semibold text-[color:var(--text-primary)]",
                    "Editor"
                }

                p {
                    class: "mt-1 truncate text-xs text-[color:var(--text-muted)]",
                    "{project.repository_full_name}"
                }
            }

            div {
                class: "min-h-0 flex-1 p-5",

                match entry {
                    None => rsx! {
                        div {
                            class: "flex h-full items-center justify-center rounded-2xl border border-white/[0.08] bg-white/[0.02] p-8 text-center",

                            p {
                                class: "max-w-sm text-sm leading-6 text-[color:var(--text-muted)]",
                                "Select a translation key to start editing."
                            }
                        }
                    },

                    Some(entry) => rsx! {
                        div {
                            class: "space-y-5",

                            div {
                                p {
                                    class: "text-xs font-semibold uppercase tracking-[0.18em] text-[color:var(--text-muted)]",
                                    "Key"
                                }

                                h1 {
                                    class: "mt-2 font-mono text-xl font-semibold text-[color:var(--text-primary)]",
                                    "{entry.key}"
                                }

                                p {
                                    class: "mt-1 text-sm text-[color:var(--text-muted)]",
                                    "{entry.file_path}"
                                }
                            }

                            div {
                                class: "rounded-2xl border border-white/[0.08] bg-white/[0.02] p-4",

                                p {
                                    class: "mb-3 text-xs font-semibold uppercase tracking-[0.18em] text-[color:var(--text-muted)]",
                                    "Current value"
                                }

                                textarea {
                                    class: "min-h-48 w-full resize-none rounded-xl border border-white/[0.08] bg-black/[0.20] p-4 font-mono text-sm leading-6 text-[color:var(--text-primary)] outline-none transition focus:border-[color:var(--bg-accent)]",
                                    value: "{entry.value}",
                                }
                            }
                        }
                    },
                }
            }
        }
    }
}