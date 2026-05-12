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
        }
    }
}

#[component]
fn OnboardingBenefit(icon: IconKind, title: &'static str, description: &'static str) -> Element {
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
