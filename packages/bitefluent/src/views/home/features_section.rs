use crate::components::{IconKind, RenderIcon};
use dioxus::prelude::*;
#[component]
pub fn FeaturesSection() -> Element {
    rsx! {
        section {
            id: "features",
            class: "mx-auto w-full max-w-7xl px-6 py-24 lg:px-10",

            div {
                class: "mx-auto max-w-2xl text-center",

                p {
                    class: "text-xs font-bold uppercase tracking-[0.22em] text-[color:var(--bg-accent)]",
                    "Built for Fluent projects"
                }

                h2 {
                    class: "mt-4 font-serif text-3xl leading-[0.95] tracking-[-0.04em] text-[color:var(--text-primary)] sm:text-4xl",
                    "Everything you need to ship translations with confidence."
                }
            }

            div {
                class: "mt-12 grid gap-5 sm:grid-cols-2 lg:grid-cols-4",

                FeatureCard {
                    icon: IconKind::Github,
                    title: "GitHub sync",
                    description: "Stay in sync with your repo. We detect changes, pull new keys, and push translations back through pull requests.",
                }

                FeatureCard {
                    icon: IconKind::Code,
                    title: "Fluent-aware editor",
                    description: "Edit translations with full context, placeables, and validation. No more guessing or breaking variables.",
                }

                FeatureCard {
                    icon: IconKind::Users,
                    title: "Review workflow",
                    description: "Collaborate with translators and reviewers. Comment, suggest, and approve changes before publishing.",
                }

                FeatureCard {
                    icon: IconKind::Chart,
                    title: "Translation status",
                    description: "Track coverage, outdated keys, and progress per language in real time across your entire project.",
                }
            }
        }
    }
}

#[component]
fn FeatureCard(icon: IconKind, title: &'static str, description: &'static str) -> Element {
    rsx! {
        article {
            class: "group flex h-full flex-col rounded-2xl border border-white/[0.08] bg-white/[0.03] p-6 shadow-2xl shadow-black/20 transition hover:border-white/[0.14] hover:bg-white/[0.045]",

            div {
                class: "flex size-10 items-center justify-center rounded-xl text-[color:var(--bg-accent)]",

                RenderIcon {
                    kind: icon,
                    class: Some("size-8".to_string()),
                }
            }

            h3 {
                class: "mt-6 text-base font-semibold tracking-[-0.02em] text-[color:var(--text-primary)]",
                "{title}"
            }

            p {
                class: "mt-3 text-sm leading-6 text-[color:var(--text-muted)]",
                "{description}"
            }

            a {
                href: "#",
                class: "mt-auto inline-flex items-center gap-2 pt-6 text-sm font-semibold text-[color:var(--bg-accent)] transition group-hover:opacity-80",

                "Learn more"

                span {
                    class: "transition group-hover:translate-x-0.5",
                    "→"
                }
            }
        }
    }
}
