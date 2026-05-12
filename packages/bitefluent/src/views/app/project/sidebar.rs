use crate::api::projects::{ProjectDto, ProjectWorkspaceFileDto};
use crate::components::{IconKind, RenderIcon};
use dioxus::prelude::*;
use std::collections::{BTreeMap, BTreeSet};

#[component]
pub fn ProjectSidebar(
    project: ProjectDto,
    files: Vec<ProjectWorkspaceFileDto>,
    selected_file: Option<String>,
    on_select_file: EventHandler<String>,
) -> Element {
    let source_locale = project.source_locale.clone();

    let source_files = files
        .into_iter()
        .filter(|file| {
            source_locale
                .as_deref()
                .map(|source_locale| locale_matches(file.locale.as_deref(), source_locale))
                .unwrap_or(true)
        })
        .collect::<Vec<_>>();

    let grouped_files = group_workspace_files(source_files);
    let mut collapsed_groups = use_signal(|| BTreeSet::<String>::new());

    rsx! {
        aside {
            class: "flex h-full min-h-0 flex-col overflow-hidden border-r border-white/[0.08] bg-black/[0.10]",

            div {
                class: "shrink-0 border-b border-white/[0.08] p-5",

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
                class: "min-h-0 flex-1 overflow-y-auto px-3 pb-3 pt-5 [scrollbar-color:rgba(255,255,255,0.16)_transparent] [scrollbar-width:thin]",

                if grouped_files.is_empty() {
                    p {
                        class: "rounded-xl border border-white/[0.08] bg-white/[0.02] p-4 text-sm leading-6 text-[color:var(--text-muted)]",
                        "No Fluent files found."
                    }
                } else {
                    div {
                        class: "space-y-3",

                        for group in grouped_files {
                            FileGroupSection {
                                group: group.clone(),
                                collapsed: collapsed_groups().contains(&group.name),
                                selected_file: selected_file.clone(),
                                on_toggle: move |group_name| {
                                    collapsed_groups.with_mut(|groups| {
                                        if groups.contains(&group_name) {
                                            groups.remove(&group_name);
                                        } else {
                                            groups.insert(group_name);
                                        }
                                    });
                                },
                                on_select_file: move |path| on_select_file.call(path),
                            }
                        }
                    }
                }
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct FileGroup {
    name: String,
    files: Vec<ProjectWorkspaceFileDto>,
}

#[component]
fn FileGroupSection(
    group: FileGroup,
    collapsed: bool,
    selected_file: Option<String>,
    on_toggle: EventHandler<String>,
    on_select_file: EventHandler<String>,
) -> Element {
    let group_name = group.name.clone();
    let file_count = group.files.len();

    let chevron_class = if collapsed {
        "h-2.5 w-2.5 shrink-0 -rotate-90 text-[color:var(--text-muted)] transition-transform"
    } else {
        "h-2.5 w-2.5 shrink-0 text-[color:var(--text-muted)] transition-transform"
    };

    rsx! {
        section {
            class: "space-y-1",

            button {
                class: "group flex w-full items-center justify-between rounded-xl px-2 py-1.5 text-left transition hover:bg-white/[0.035]",
                type: "button",
                onclick: move |_| {
                    on_toggle.call(group_name.clone());
                },

                span {
                    class: "flex min-w-0 items-center gap-2",

                    RenderIcon {
                        kind: IconKind::ChevronDown,
                        class: Some(chevron_class.to_string()),
                    }

                    RenderIcon {
                        kind: IconKind::Folder,
                        class: Some("h-3.5 w-3.5 shrink-0 text-[color:var(--text-muted)] transition group-hover:text-[color:var(--text-secondary)]".to_string()),
                    }

                    span {
                        class: "truncate text-xs font-semibold uppercase tracking-[0.14em] text-[color:var(--text-muted)] transition group-hover:text-[color:var(--text-secondary)]",
                        "{group.name}"
                    }
                }

                span {
                    class: "shrink-0 rounded-full bg-white/[0.04] px-1.5 py-0.5 text-[10px] font-medium text-[color:var(--text-muted)]",
                    "{file_count}"
                }
            }

            if !collapsed {
                div {
                    class: "space-y-1",

                    for file in group.files {
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

#[component]
fn FileButton(
    file: ProjectWorkspaceFileDto,
    selected: bool,
    on_select: EventHandler<String>,
) -> Element {
    let class = if selected {
        "group flex w-full items-center gap-3 rounded-2xl border border-white/[0.10] bg-white/[0.055] px-3 py-2.5 text-left shadow-sm shadow-black/10"
    } else {
        "group flex w-full items-center gap-3 rounded-2xl border border-transparent px-3 py-2.5 text-left transition hover:border-white/[0.08] hover:bg-white/[0.04]"
    };

    let icon_wrapper_class = if selected {
        "flex h-8 w-8 shrink-0 items-center justify-center rounded-xl bg-white/[0.06]"
    } else {
        "flex h-8 w-8 shrink-0 items-center justify-center rounded-xl bg-white/[0.035] transition group-hover:bg-white/[0.06]"
    };

    let icon_class = if selected {
        "h-4 w-4 shrink-0 text-[color:var(--text-secondary)]"
    } else {
        "h-4 w-4 shrink-0 text-[color:var(--text-muted)] transition group-hover:text-[color:var(--text-secondary)]"
    };

    let name_class = if selected {
        "block truncate text-sm font-semibold text-[color:var(--text-primary)]"
    } else {
        "block truncate text-sm font-medium text-[color:var(--text-primary)]"
    };

    rsx! {
        button {
            class: "{class}",
            type: "button",
            title: "{file.path}",
            onclick: {
                let path = file.path.clone();

                move |_| {
                    on_select.call(path.clone());
                }
            },

            span {
                class: "{icon_wrapper_class}",

                RenderIcon {
                    kind: IconKind::FileText,
                    class: Some(icon_class.to_string()),
                }
            }

            span {
                class: "min-w-0 flex-1",

                span {
                    class: "{name_class}",
                    "{file.name}"
                }
            }
        }
    }
}

fn group_workspace_files(files: Vec<ProjectWorkspaceFileDto>) -> Vec<FileGroup> {
    let mut groups = BTreeMap::<String, Vec<ProjectWorkspaceFileDto>>::new();

    for file in files {
        let group_name = file_group_label(&file.path, &file.name);
        groups.entry(group_name).or_default().push(file);
    }

    groups
        .into_iter()
        .map(|(name, mut files)| {
            files.sort_by(|left, right| {
                left.name
                    .cmp(&right.name)
                    .then_with(|| left.path.cmp(&right.path))
            });

            FileGroup { name, files }
        })
        .collect()
}

fn file_group_label(path: &str, name: &str) -> String {
    let parent = path
        .strip_suffix(name)
        .unwrap_or(path)
        .trim_end_matches('/');

    let parts = parent
        .split('/')
        .filter(|part| !part.is_empty())
        .filter(|part| !looks_like_locale(part))
        .filter(|part| {
            !matches!(
                *part,
                "locales" | "locale" | "i18n" | "translations" | "translation" | "lang" | "langs"
            )
        })
        .collect::<Vec<_>>();

    parts
        .last()
        .map(|part| humanize_group_name(part))
        .filter(|name| !name.is_empty())
        .unwrap_or_else(|| "General".to_string())
}

fn humanize_group_name(value: &str) -> String {
    value.replace('_', " ").replace('-', " ")
}

fn looks_like_locale(value: &str) -> bool {
    let normalized = value.replace('_', "-");
    let parts = normalized.split('-').collect::<Vec<_>>();

    match parts.as_slice() {
        [language] => language.len() == 2 || language.len() == 3,
        [language, region] => {
            (language.len() == 2 || language.len() == 3) && (region.len() == 2 || region.len() == 4)
        }
        _ => false,
    }
}

fn locale_matches(file_locale: Option<&str>, source_locale: &str) -> bool {
    let Some(file_locale) = file_locale else {
        return false;
    };

    normalize_locale(file_locale) == normalize_locale(source_locale)
}

fn normalize_locale(locale: &str) -> String {
    locale.replace('_', "-").to_lowercase()
}
