use crate::api::auth::AuthUserDto;
use crate::api::github::{fetch_github_repositories, GithubRepositoryDto};
use crate::components::{
    Button, ButtonSize, ButtonVariant, Icon, IconKind, IconPosition, RenderIcon, Select,
    SelectOption,
};
use dioxus::prelude::*;

#[component]
pub fn Onboarding(user: AuthUserDto) -> Element {
    let mut repositories = use_resource(fetch_github_repositories);
    let selected_repository_id = use_signal(|| None::<u64>);
    let selected_owner = use_signal(|| None::<String>);
    let repository_state = repositories.read().clone();

    rsx! {
        section {
            class: "mx-auto grid min-h-screen w-full max-w-7xl items-center gap-16 px-6 py-28 lg:grid-cols-[0.85fr_1.15fr] lg:px-10",

            OnboardingIntro {}

            RepositoryCard {
                user,
                repository_state,
                selected_repository_id,
                selected_owner,
                on_reload: move |_| repositories.restart(),
            }
        }
    }
}

#[component]
fn OnboardingIntro() -> Element {
    rsx! {
        div {
            class: "max-w-2xl",

            p {
                class: "text-sm font-bold uppercase tracking-[0.22em] text-[color:var(--bg-accent)]",
                "Step 1 of 2"
            }

            h1 {
                class: "mt-6 font-serif text-5xl leading-[0.95] tracking-[-0.04em] text-[color:var(--text-primary)] sm:text-6xl lg:text-7xl",
                "Connect your"
                br {}
                "GitHub repository"
                span {
                    class: "text-[color:var(--bg-accent)]",
                    "."
                }
            }

            p {
                class: "mt-7 max-w-xl text-lg leading-8 text-[color:var(--text-secondary)]",
                "Import a repository to start discovering and translating your Fluent files."
            }

            div {
                class: "mt-14 space-y-8",

                OnboardingBenefit {
                    icon: IconKind::Github,
                    title: "Secure connection",
                    description: "We only read repository content.",
                }

                OnboardingBenefit {
                    icon: IconKind::Zap,
                    title: "Quick and easy",
                    description: "You can switch repositories anytime.",
                }
            }

            div {
                class: "mt-24 flex items-center gap-3 text-sm text-[color:var(--text-muted)]",

                span {
                    class: "inline-flex size-5 items-center justify-center rounded-md border border-white/[0.12] text-[color:var(--text-muted)]",

                    RenderIcon {
                        kind: IconKind::Lock,
                        class: Some("size-3".to_string()),
                    }
                }

                "BiteFluent is read-only and secure."
            }
        }
    }
}

#[component]
fn RepositoryCard(
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
                    RepositorySkeleton {}
                    RepositorySkeleton {}
                    RepositorySkeleton {}
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
        "error" => "rounded-xl border border-red-400/20 bg-red-500/10 p-4 text-sm text-red-200",
        _ => "rounded-xl border border-white/[0.08] bg-white/[0.02] p-4 text-sm leading-6 text-[color:var(--text-muted)]",
    };

    rsx! {
        div {
            class: "{class}",
            "{message}"
        }
    }
}

#[component]
fn OnboardingBenefit(
    icon: IconKind,
    title: &'static str,
    description: &'static str,
) -> Element {
    rsx! {
        div {
            class: "flex items-center gap-5",

            div {
                class: "flex size-14 shrink-0 items-center justify-center rounded-2xl border border-[color:var(--bg-accent)] bg-[var(--bg-accent)]/10 text-[color:var(--bg-accent)] opacity-90",

                RenderIcon {
                    kind: icon,
                    class: Some("size-6".to_string()),
                }
            }

            div {
                h3 {
                    class: "text-base font-semibold text-[color:var(--text-primary)]",
                    "{title}"
                }

                p {
                    class: "mt-1 text-sm leading-6 text-[color:var(--text-muted)]",
                    "{description}"
                }
            }
        }
    }
}

#[component]
fn RepositoryRow(
    repository: GithubRepositoryDto,
    selected: bool,
    on_select: EventHandler<u64>,
) -> Element {
    let row_class = if selected {
        "border-[color:var(--bg-accent)] bg-[var(--bg-accent)]/[0.06]"
    } else {
        "border-white/[0.08] bg-white/[0.02] hover:border-white/[0.14] hover:bg-white/[0.035]"
    };

    let check_class = if selected {
        "flex size-6 shrink-0 items-center justify-center rounded-full bg-[var(--bg-accent)] text-xs font-bold text-white"
    } else {
        "flex size-6 shrink-0 items-center justify-center rounded-full border border-white/[0.16] bg-white/[0.02]"
    };

    let select_class = if selected {
        "rounded-lg border border-[color:var(--bg-accent)] px-4 py-2 text-sm font-semibold text-[color:var(--bg-accent)]"
    } else {
        "rounded-lg border border-white/[0.10] px-4 py-2 text-sm font-semibold text-[color:var(--text-secondary)] transition group-hover:border-white/[0.18] group-hover:text-[color:var(--text-primary)]"
    };

    let visibility = if repository.private { "Private" } else { "Public" };
    let repository_id = repository.id;

    rsx! {
        button {
            class: "group flex h-16 w-full items-center gap-4 rounded-xl border px-4 text-left transition {row_class}",
            type: "button",
            onclick: move |_| on_select.call(repository_id),

            span {
                class: "{check_class}",

                if selected {
                    RenderIcon {
                        kind: IconKind::CheckNoCircle,
                        class: Some("size-3".to_string()),
                    }
                }
            }

            RenderIcon {
                kind: IconKind::Repository,
                class: Some("size-5 shrink-0 text-[color:var(--text-muted)]".to_string()),
            }

            span {
                class: "min-w-0 flex-1 truncate text-sm font-semibold text-[color:var(--text-primary)]",
                "{repository.full_name}"
            }

            span {
                class: "rounded-full bg-white/[0.06] px-2.5 py-1 text-xs font-medium text-[color:var(--text-muted)]",
                "{visibility}"
            }
        }
    }
}

#[component]
fn RepositorySkeleton() -> Element {
    rsx! {
        div {
            class: "flex h-16 w-full items-center gap-4 rounded-xl border border-white/[0.08] bg-white/[0.02] px-4",

            span { class: "size-6 shrink-0 animate-pulse rounded-full bg-white/[0.06]" }
            span { class: "size-5 shrink-0 animate-pulse rounded bg-white/[0.06]" }
            span { class: "h-4 flex-1 animate-pulse rounded bg-white/[0.06]" }
            span { class: "h-7 w-16 animate-pulse rounded-full bg-white/[0.06]" }
            span { class: "h-9 w-20 animate-pulse rounded-lg bg-white/[0.06]" }
        }
    }
}

fn repository_owners(repositories: &[GithubRepositoryDto]) -> Vec<String> {
    let mut owners = repositories
        .iter()
        .map(|repository| repository.owner.clone())
        .collect::<Vec<_>>();

    owners.sort();
    owners.dedup();

    owners
}

fn effective_owner(
    repository_state: &Option<ServerFnResult<Vec<GithubRepositoryDto>>>,
    selected_owner: Signal<Option<String>>,
    fallback: String,
) -> String {
    selected_owner().unwrap_or_else(|| {
        repository_state
            .as_ref()
            .and_then(|state| state.as_ref().ok())
            .map(|repositories| repository_owners(repositories))
            .and_then(|owners| owners.first().cloned())
            .unwrap_or(fallback)
    })
}