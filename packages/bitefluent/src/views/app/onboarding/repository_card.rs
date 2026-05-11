use super::helpers::{
    effective_owner, placeholder_repository, repository_owners,
};
use super::repository_row::RepositoryRow;
use crate::api::auth::AuthUserDto;
use crate::api::github::GithubRepositoryDto;
use crate::components::{
    Button, ButtonSize, ButtonVariant, Icon, IconKind, IconPosition, RenderIcon, Select,
    SelectOption, Skeleton,
};
use dioxus::prelude::*;

#[component]
pub fn RepositoryCard(
    user: AuthUserDto,
    repository_state: Option<ServerFnResult<Vec<GithubRepositoryDto>>>,
    selected_repository_id: Signal<Option<u64>>,
    selected_owner: Signal<Option<String>>,
    on_reload: EventHandler<MouseEvent>,
) -> Element {
    rsx! {
        div {
            class: "w-full max-w-2xl justify-self-end",

            div {
                class: "rounded-3xl border border-white/[0.10] bg-white/[0.035] p-6 shadow-2xl shadow-black/30 backdrop-blur sm:p-8",

                RepositoryCardHeader {}

                OwnerSelect {
                    user,
                    repository_state: repository_state.clone(),
                    selected_owner,
                    selected_repository_id,
                }

                RepositorySearch {}

                RepositoryList {
                    repository_state,
                    selected_repository_id,
                    selected_owner,
                }

                div {
                    class: "mt-6",

                    Button {
                        label: "Import selected repository",
                        variant: ButtonVariant::Primary,
                        size: ButtonSize::Lg,
                        full_width: true,
                        balanced: true,
                        class: Some("h-14 px-6 shadow-lg shadow-[color:var(--bg-accent)]/20".to_string()),
                        icon: Some(Icon {
                            kind: IconKind::Arrow,
                            position: IconPosition::Right,
                            class: Some("size-4".to_string()),
                        }),
                    }
                }
            }

            p {
                class: "mt-5 flex items-center justify-center gap-1.5 text-center text-sm text-[color:var(--text-muted)]",

                "Can't find your repository? Make sure it's accessible and "

                button {
                    class: "inline-flex items-center gap-1.5 font-medium text-[color:var(--bg-accent)] transition hover:opacity-80",
                    type: "button",
                    onclick: move |event| on_reload.call(event),

                    RenderIcon {
                        kind: IconKind::Refresh,
                        class: Some("size-3".to_string()),
                    }

                    "try reloading"
                }

                "."
            }
        }
    }
}

#[component]
fn OwnerSelect(
    user: AuthUserDto,
    repository_state: Option<ServerFnResult<Vec<GithubRepositoryDto>>>,
    selected_owner: Signal<Option<String>>,
    selected_repository_id: Signal<Option<u64>>,
) -> Element {
    let owners = repository_state
        .as_ref()
        .and_then(|state| state.as_ref().ok())
        .map(|repositories| repository_owners(repositories))
        .unwrap_or_default();

    let current_owner = effective_owner(
        &repository_state,
        selected_owner,
        user.display_name.clone(),
    );

    let initial = current_owner.chars().next().unwrap_or('?');

    rsx! {
        Select {
            label: "GitHub owner",
            value: current_owner,
            class: Some("mt-8".to_string()),
            options: owners
                .into_iter()
                .map(|owner| SelectOption {
                    label: owner.clone(),
                    value: owner,
                })
                .collect(),
            leading: Some(rsx! {
                span {
                    class: "flex size-7 shrink-0 items-center justify-center rounded-full bg-[var(--bg-accent)] text-xs font-bold text-white",
                    "{initial}"
                }
            }),
            on_change: move |owner| {
                selected_owner.set(Some(owner));
                selected_repository_id.set(None);
            },
        }
    }
}

#[component]
fn RepositoryCardHeader() -> Element {
    rsx! {
        div {
            class: "flex items-start justify-between gap-6",

            div {
                h2 {
                    class: "font-serif text-3xl leading-none tracking-[-0.04em] text-[color:var(--text-primary)]",
                    "Import your repository"
                }

                p {
                    class: "mt-3 text-sm text-[color:var(--text-secondary)]",
                    "Select a repository to import and start translating."
                }
            }

            div {
                class: "flex items-center gap-2 pt-1",

                span {
                    class: "flex size-7 items-center justify-center rounded-full bg-[var(--bg-accent)] text-xs font-bold text-white shadow-lg shadow-[color:var(--bg-accent)]/20",
                    "1"
                }

                span {
                    class: "h-px w-8 bg-[var(--bg-accent)]/40",
                }

                span {
                    class: "flex size-7 items-center justify-center rounded-full border border-white/[0.12] bg-white/[0.04] text-xs font-bold text-[color:var(--text-muted)]",
                    "2"
                }
            }
        }
    }
}

#[component]
fn RepositorySearch() -> Element {
    rsx! {
        div {
            class: "mt-4 flex h-12 items-center gap-3 rounded-xl border border-white/[0.10] bg-white/[0.025] px-4",

            RenderIcon {
                kind: IconKind::Search,
                class: Some("size-4 shrink-0 text-[color:var(--text-muted)]".to_string()),
            }

            input {
                class: "h-full w-full bg-transparent text-sm text-[color:var(--text-primary)] outline-none placeholder:text-[color:var(--text-muted)]",
                placeholder: "Search repositories...",
            }
        }
    }
}

#[component]
fn RepositoryList(
    repository_state: Option<ServerFnResult<Vec<GithubRepositoryDto>>>,
    selected_repository_id: Signal<Option<u64>>,
    selected_owner: Signal<Option<String>>,
) -> Element {
    let current_owner = repository_state
        .as_ref()
        .and_then(|state| state.as_ref().ok())
        .and_then(|repositories| {
            selected_owner()
                .or_else(|| repository_owners(repositories).first().cloned())
        });

    let visible_repositories = match repository_state.as_ref() {
        Some(Ok(repositories)) => repositories
            .iter()
            .filter(|repository| {
                current_owner
                    .as_ref()
                    .map(|owner| repository.owner == *owner)
                    .unwrap_or(true)
            })
            .take(5)
            .cloned()
            .collect::<Vec<_>>(),
        _ => Vec::new(),
    };

    let selected_id = selected_repository_id();
    let should_select_first = selected_id
        .map(|id| !visible_repositories.iter().any(|repo| repo.id == id))
        .unwrap_or(true);

    rsx! {
        div {
            class: "mt-4 space-y-2",

            match repository_state.as_ref() {
                None => rsx! {
                    for index in 0..3 {
                        Skeleton {
                            suspend: true,

                            RepositoryRow {
                                selected: false,
                                repository: placeholder_repository(index),
                                on_select: move |_| {},
                            }
                        }
                    }
                },

                Some(Err(_)) => rsx! {
                    StateMessage {
                        tone: "error",
                        message: "Could not load repositories. Try reloading.",
                    }
                },

                Some(Ok(_)) if visible_repositories.is_empty() => rsx! {
                    StateMessage {
                        tone: "muted",
                        message: "No repositories found for this owner.",
                    }
                },

                Some(Ok(_)) => rsx! {
                    for (index, repository) in visible_repositories.into_iter().enumerate() {
                        RepositoryRow {
                            selected: selected_id
                                .map(|id| id == repository.id)
                                .unwrap_or(false) || (should_select_first && index == 0),
                            repository,
                            on_select: move |repository_id| {
                                selected_repository_id.set(Some(repository_id));
                            },
                        }
                    }
                },
            }
        }
    }
}

#[component]
fn StateMessage(tone: &'static str, message: &'static str) -> Element {
    let class = match tone {
        "error" => {
            "rounded-xl border border-red-400/20 bg-red-500/10 p-4 text-sm text-red-200"
        }
        _ => {
            "rounded-xl border border-white/[0.08] bg-white/[0.02] p-4 text-sm leading-6 text-[color:var(--text-muted)]"
        }
    };

    rsx! {
        div {
            class: "{class}",
            "{message}"
        }
    }
}