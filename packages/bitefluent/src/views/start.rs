use crate::api::auth::AuthUserResource;
use crate::components::{Button, ButtonSize, ButtonVariant, Icon, IconKind, IconPosition, MetaFor};
use dioxus::prelude::*;

#[component]
pub fn Start() -> Element {
    let current_user = use_context::<AuthUserResource>();
    let user_state = current_user.read().clone();
    let is_authenticated = matches!(user_state.as_ref(), Some(Some(_)));
    let navigator = use_navigator();

    use_effect(use_reactive(
        (&is_authenticated,),
        move |(is_authenticated,)| {
            if is_authenticated {
                let _ = navigator.replace("/app");
            }
        },
    ));

    if is_authenticated {
        return rsx! {
            div {
                class: "min-h-screen bg-[image:var(--bg-main)]"
            }
        };
    }

    rsx! {
        crate::components::Meta { r#for: MetaFor::Start }

        section {
            class: "relative mx-auto grid min-h-screen w-full max-w-7xl items-center gap-12 px-6 pb-24 pt-32 lg:grid-cols-[0.9fr_1fr] lg:px-10",

            div {
                class: "pointer-events-none absolute right-[-12rem] top-24 -z-10 size-80 rounded-full bg-violet-500/10 blur-3xl",
            }

            div {
                class: "max-w-2xl",

                h1 {
                    class: "font-serif text-5xl leading-[0.95] tracking-[-0.04em] text-[color:var(--text-primary)] sm:text-6xl lg:text-7xl",
                    "Start with"
                    br {}
                    "BiteFluent"
                    span {
                        class: "text-[color:var(--bg-accent)]",
                        "."
                    }
                }

                p {
                    class: "mt-8 max-w-md text-lg leading-8 text-[color:var(--text-secondary)]",
                    "Sign in to create your first project and connect a repository."
                }
            }

            div {
                class: "w-full max-w-xl justify-self-center rounded-3xl border border-white/10 bg-white/[0.035] p-6 shadow-2xl shadow-black/30 backdrop-blur sm:p-10",

                div {
                    class: "text-center",

                    h2 {
                        class: "text-2xl font-semibold tracking-[-0.02em] text-[color:var(--text-primary)]",
                        "Get started"
                    }

                    p {
                        class: "mt-3 text-base text-[color:var(--text-secondary)]",
                        "Sign in to continue to BiteFluent."
                    }
                }

                div {
                    class: "mt-8 flex flex-col gap-3",

                    Button {
                        label: "Continue with GitHub",
                        href: Some("/auth/signin/github"),
                        variant: ButtonVariant::Primary,
                        size: ButtonSize::Lg,
                        full_width: true,
                        balanced: true,
                        class: Some("h-14 px-6 shadow-lg shadow-[color:var(--bg-accent)]/20".to_string()),
                        icon: Some(Icon {
                            kind: IconKind::Github,
                            position: IconPosition::Left,
                            class: Some("size-6".to_string()),
                        }),
                    }

                    Button {
                        label: "Continue with Discord",
                        href: Some("/auth/signin/discord"),
                        variant: ButtonVariant::Secondary,
                        size: ButtonSize::Lg,
                        full_width: true,
                        balanced: true,
                        class: Some("h-14 px-6".to_string()),
                        icon: Some(Icon {
                            kind: IconKind::Discord,
                            position: IconPosition::Left,
                            class: Some("size-6 text-[color:var(--bg-accent)]".to_string()),
                        }),
                    }

                    Button {
                        label: "Continue with Google",
                        href: Some("/auth/signin/google"),
                        variant: ButtonVariant::Secondary,
                        size: ButtonSize::Lg,
                        full_width: true,
                        balanced: true,
                        class: Some("h-14 px-6".to_string()),
                        icon: Some(Icon {
                            kind: IconKind::Google,
                            position: IconPosition::Left,
                            class: Some("size-6".to_string()),
                        }),
                    }
                }
            }
        }
    }
}
