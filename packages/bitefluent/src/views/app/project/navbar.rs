use crate::api::auth::AuthUserResource;
use crate::components::{
    Button, ButtonSize, ButtonVariant, Icon, IconKind, IconPosition, RenderIcon,
};
use dioxus::prelude::*;

const LOGO: Asset = asset!("/assets/logo.svg");

#[component]
pub fn ProjectNavbar() -> Element {
    let current_user = use_context::<AuthUserResource>();
    let user_state = current_user.read().clone();

    rsx! {
        header {
            class: "fixed inset-x-0 top-0 z-50 border-b border-white/[0.08] bg-[rgba(10,10,14,0.78)] backdrop-blur-xl",

            div {
                class: "mx-auto flex h-16 w-full max-w-[1800px] items-center justify-between px-4 lg:px-6",

                div {
                    class: "flex min-w-0 items-center gap-4",

                    Link {
                        to: "/app",
                        class: "flex items-center gap-3 shrink-0",

                        img {
                            src: LOGO,
                            alt: "BiteFluent Logo",
                            class: "h-6 w-6",
                        }

                        span {
                            class: "font-serif text-xl font-medium tracking-[-0.03em] text-[color:var(--text-primary)]",
                            "BiteFluent"
                        }
                    }

                    div {
                        class: "hidden h-6 w-px bg-white/[0.10] md:block",
                    }

                    div {
                        class: "hidden min-w-0 items-center gap-2 text-sm text-[color:var(--text-muted)] md:flex",

                        Link {
                            to: "/app",
                            class: "transition hover:text-[color:var(--text-primary)]",
                            "Projects"
                        }

                        span {
                            class: "text-[color:var(--text-muted)]/60",
                            "/"
                        }

                        span {
                            class: "truncate font-medium text-[color:var(--text-primary)]",
                            "Project"
                        }
                    }
                }

                div {
                    class: "flex items-center gap-3",

                    Button {
                        label: "Sync repository",
                        variant: ButtonVariant::Secondary,
                        size: ButtonSize::Sm,
                        icon: Some(Icon {
                            kind: IconKind::Refresh,
                            position: IconPosition::Left,
                            class: Some("size-4".to_string()),
                        }),
                    }

                    button {
                        class: "hidden h-10 items-center gap-2 rounded-lg px-3 text-sm font-medium text-[color:var(--text-secondary)] transition hover:bg-white/[0.05] hover:text-[color:var(--text-primary)] md:inline-flex",
                        type: "button",

                        RenderIcon {
                            kind: IconKind::Settings,
                            class: Some("size-4".to_string()),
                        }

                        "Settings"
                    }

                    div {
                        class: "h-6 w-px bg-white/[0.10]",
                    }

                    match user_state.as_ref() {
                        Some(Some(user)) => rsx! {
                            ProjectUserBadge {
                                name: user.display_name.clone(),
                                image: user.image.clone(),
                            }
                        },

                        Some(None) => rsx! {
                            Link {
                                to: "/start",
                                class: "rounded-lg border border-white/[0.12] bg-white/[0.03] px-4 py-2 text-sm font-medium text-white transition hover:bg-white/[0.06]",
                                "Sign in"
                            }
                        },

                        None => rsx! {
                            div {
                                class: "h-10 w-28 animate-pulse rounded-lg bg-white/[0.04]",
                            }
                        },
                    }
                }
            }
        }
    }
}

#[component]
fn ProjectUserBadge(name: Option<String>, image: Option<String>) -> Element {
    let fallback = name
        .as_deref()
        .and_then(|name| name.chars().next())
        .unwrap_or('?');

    let alt = name.as_deref().unwrap_or("User avatar").to_string();

    rsx! {
        Link {
            to: "/app",
            class: "flex h-10 items-center gap-3 rounded-lg border border-white/[0.12] bg-white/[0.03] px-3 text-sm font-medium text-white transition hover:border-white/[0.20] hover:bg-white/[0.06]",

            span {
                class: "flex size-6 items-center justify-center overflow-hidden rounded-full border border-white/[0.12] bg-white/[0.04] text-xs font-semibold text-[color:var(--text-primary)]",

                if let Some(image) = image.as_ref() {
                    img {
                        src: "{image}",
                        alt: "{alt}",
                        class: "size-full object-cover",
                    }
                } else {
                    span {
                        class: "uppercase",
                        "{fallback}"
                    }
                }
            }

            span {
                class: "hidden sm:block",
                "Account"
            }
        }
    }
}
