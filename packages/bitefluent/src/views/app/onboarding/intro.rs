use crate::components::{IconKind, RenderIcon};
use dioxus::prelude::*;

#[component]
pub fn OnboardingIntro() -> Element {
    rsx! {
        div {
            class: "max-w-2xl",

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