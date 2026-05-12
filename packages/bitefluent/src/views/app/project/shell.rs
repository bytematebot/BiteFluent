use crate::api::projects::{fetch_project_workspace, ProjectWorkspaceDto};
use crate::components::{MetaFor, Skeleton};
use dioxus::prelude::*;

use super::editor::ProjectEditorPanel;
use super::keys::ProjectKeysPanel;
use super::sidebar::ProjectSidebar;

#[component]
pub fn AppProject(project_id: String) -> Element {
    let project_id_for_fetch = project_id.clone();

    let workspace = use_resource(move || {
        let project_id = project_id_for_fetch.clone();

        async move { fetch_project_workspace(project_id).await.ok().flatten() }
    });

    let selected_file = use_signal(|| None::<String>);
    let selected_key = use_signal(|| None::<String>);

    rsx! {
        crate::components::Meta { r#for: MetaFor::Home }

        section {
            class: "mx-auto flex min-h-screen w-full max-w-[1800px] flex-col px-4 pt-20 pb-4 lg:px-6",

            match workspace.read().as_ref() {
                None => rsx! {
                    ProjectWorkspaceSkeleton {}
                },

                Some(None) => rsx! {
                    ProjectNotFound {}
                },

                Some(Some(workspace)) => rsx! {
                    ProjectWorkspace {
                        workspace: workspace.clone(),
                        selected_file,
                        selected_key,
                    }
                },
            }
        }
    }
}

#[component]
fn ProjectWorkspace(
    workspace: ProjectWorkspaceDto,
    selected_file: Signal<Option<String>>,
    selected_key: Signal<Option<String>>,
) -> Element {
    let current_file = selected_file()
        .or_else(|| workspace.files.first().map(|file| file.path.clone()));

    let visible_keys = current_file
        .as_ref()
        .map(|file_path| {
            workspace
                .keys
                .iter()
                .filter(|key| key.file_path == *file_path)
                .cloned()
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();

    let current_key = selected_key()
        .or_else(|| visible_keys.first().map(|key| key.key.clone()));

    let selected_entry = current_key
        .as_ref()
        .and_then(|key_name| {
            visible_keys
                .iter()
                .find(|key| &key.key == key_name)
                .cloned()
        });

    rsx! {
        div {
            class: "grid min-h-[calc(100vh-6rem)] overflow-hidden rounded-3xl border border-white/[0.10] bg-white/[0.025] shadow-2xl shadow-black/30 lg:grid-cols-[18rem_minmax(22rem,30rem)_1fr]",

            ProjectSidebar {
                project: workspace.project.clone(),
                files: workspace.files.clone(),
                selected_file: current_file.clone(),
                on_select_file: move |path| {
                    selected_file.set(Some(path));
                    selected_key.set(None);
                },
            }

            ProjectKeysPanel {
                keys: visible_keys,
                selected_key: current_key,
                on_select_key: move |key| {
                    selected_key.set(Some(key));
                },
            }

            ProjectEditorPanel {
                project: workspace.project,
                entry: selected_entry,
            }
        }
    }
}

#[component]
fn ProjectWorkspaceSkeleton() -> Element {
    rsx! {
        div {
            class: "grid min-h-[calc(100vh-6rem)] overflow-hidden rounded-3xl border border-white/[0.10] bg-white/[0.025] lg:grid-cols-[18rem_minmax(22rem,30rem)_1fr]",

            for _ in 0..3 {
                Skeleton {
                    suspend: true,
                    class: "min-h-[calc(100vh-6rem)] bg-white/[0.03]".to_string(),
                    div {}
                }
            }
        }
    }
}

#[component]
fn ProjectNotFound() -> Element {
    rsx! {
        div {
            class: "pt-24",

            h1 {
                class: "font-serif text-5xl leading-none tracking-[-0.04em] text-[color:var(--text-primary)]",
                "Project not found."
            }

            p {
                class: "mt-5 text-lg text-[color:var(--text-secondary)]",
                "This project does not exist or you do not have access to it."
            }
        }
    }
}