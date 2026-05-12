use crate::api::auth::AuthUserDto;
use crate::api::projects::{ProjectDto, fetch_projects};
use crate::components::{
    Button, ButtonSize, ButtonVariant, Icon, IconKind, IconPosition, MetaFor, RenderIcon, Skeleton,
};
use dioxus::prelude::*;

#[component]
pub fn AppDashboard(user: AuthUserDto) -> Element {
    let projects = use_resource(fetch_projects);
    let projects_state = projects.read().clone();

    rsx! {
        crate::components::Meta { r#for: MetaFor::Home }

        section {
            class: "mx-auto flex min-h-screen w-full max-w-7xl flex-col px-6 pt-36 pb-24 lg:px-30",

            div {
                class: "flex items-start justify-between gap-8",

                div {
                    p {
                        class: "text-sm font-bold uppercase tracking-[0.22em] text-[color:var(--bg-accent)]",
                        "BiteFluent app"
                    }

                    h1 {
                        class: "mt-4 font-serif text-4xl leading-none tracking-[-0.04em] text-[color:var(--text-primary)] sm:text-5xl",
                        "Projects"
                        span {
                            class: "text-[color:var(--bg-accent)]",
                            "."
                        }
                    }

                    p {
                        class: "mt-5 max-w-xl text-lg leading-8 text-[color:var(--text-secondary)]",
                        "Manage translation projects connected to your repositories."
                    }
                }

                Button {
                    label: "New project",
                    variant: ButtonVariant::Primary,
                    size: ButtonSize::Md,
                    icon: Some(Icon {
                        kind: IconKind::Arrow,
                        position: IconPosition::Right,
                        class: Some("size-4".to_string()),
                    }),
                    to: "/app/new",
                }
            }

            div {
                class: "mt-8",

                match projects_state.as_ref() {
                    None => rsx! {
                        DashboardSkeleton {}
                    },

                    Some(Err(_)) => rsx! {
                        DashboardMessage {
                            title: "Could not load projects",
                            description: "Try refreshing the page."
                        }
                    },

                    Some(Ok(projects)) if projects.is_empty() => rsx! {
                        DashboardMessage {
                            title: "No projects yet",
                            description: "Create your first project to start translating."
                        }
                    },

                    Some(Ok(projects)) => rsx! {
                        div {
                            class: "grid gap-4 md:grid-cols-2 xl:grid-cols-3",

                            for project in projects.iter() {
                                ProjectCard {
                                    project: project.clone(),
                                }
                            }
                        }
                    },
                }
            }
        }
    }
}

#[component]
fn ProjectCard(project: ProjectDto) -> Element {
    rsx! {
        div {
            class: "rounded-3xl border border-white/[0.10] bg-white/[0.035] p-6 shadow-xl shadow-black/20 transition hover:border-white/[0.16] hover:bg-white/[0.05]",

            div {
                class: "flex items-start gap-4",

                div {
                    class: "flex size-11 shrink-0 items-center justify-center rounded-2xl border border-[color:var(--bg-accent)] bg-[var(--bg-accent)]/10 text-[color:var(--bg-accent)]",

                    RenderIcon {
                        kind: IconKind::Repository,
                        class: Some("size-5".to_string()),
                    }
                }

                div {
                    class: "min-w-0 flex-1",

                    h2 {
                        class: "truncate text-base font-semibold text-[color:var(--text-primary)]",
                        "{project.name}"
                    }

                    p {
                        class: "mt-1 truncate text-sm text-[color:var(--text-muted)]",
                        "{project.repository_full_name}"
                    }
                }
            }

            div {
                class: "mt-6",

                Button {
                    label: "Open project",
                    variant: ButtonVariant::Secondary,
                    size: ButtonSize::Md,
                    full_width: true,
                    balanced: true,
                    to: format!("/app/projects/{}", project.id),
                }
            }
        }
    }
}
#[component]
fn DashboardSkeleton() -> Element {
    rsx! {
        div {
            class: "grid gap-4 md:grid-cols-2 xl:grid-cols-3",

            for _ in 0..3 {
                Skeleton {
                    suspend: true,
                    class: "h-56 rounded-3xl border border-white/[0.08] bg-white/[0.03]".to_string(),
                    div {}
                }
            }
        }
    }
}

#[component]
fn DashboardMessage(title: &'static str, description: &'static str) -> Element {
    rsx! {
        div {
            class: "rounded-3xl border border-white/[0.08] bg-white/[0.03] p-8",

            h2 {
                class: "text-lg font-semibold text-[color:var(--text-primary)]",
                "{title}"
            }

            p {
                class: "mt-2 text-sm leading-6 text-[color:var(--text-muted)]",
                "{description}"
            }
        }
    }
}
