use crate::components::{Button, ButtonSize, ButtonVariant, Icon, IconKind, IconPosition};
use crate::views::home::trust_item::TrustItem;
use dioxus::prelude::*;

#[component]
pub fn Hero() -> Element {
    let router = use_navigator();

    rsx! {
        section {
            class: "mx-auto flex min-h-[70vh] w-full max-w-7xl items-center px-6 py-24 lg:px-10",

            div {
                class: "grid w-full items-center gap-16 lg:grid-cols-[0.9fr_1.5fr]",
                div {
                    class: "max-w-2xl",

                    h1 {
                        class: "font-serif text-5xl leading-[0.95] tracking-[-0.04em] text-[color:var(--text-primary)] sm:text-6xl lg:text-7xl",
                        "Fluent-native translation management."
                    }

                    p {
                        class: "mt-7 max-w-xl text-lg leading-8 text-[color:var(--text-secondary)]",
                        "Sync source keys, manage translations, review changes, and publish translations with a clean workflow built for fluent projects."
                    }

                    div {
                        class: "mt-10 flex flex-col gap-4 sm:flex-row",

                        Button {
                            label: "Get Started",
                            icon: Some(Icon {
                                kind: IconKind::Arrow,
                                position: IconPosition::Left,
                                class: None,
                            }),
                            variant: ButtonVariant::Primary,
                            size: ButtonSize::Lg,
                            to: Some("/start")
                        }

                        Button {
                            label: "View on GitHub",
                            variant: ButtonVariant::Secondary,
                            href: Some("https://github.com/bytematebot/bitefluent"),
                            icon: Some(Icon {
                                kind: IconKind::Github,
                                position: IconPosition::Left,
                                class: Some("w-5 h-5".to_string()),
                            })
                        }
                    }
                    div {
                        class: "mt-8 flex flex-wrap items-center gap-x-6 gap-y-3",

                        TrustItem {
                            icon: IconKind::Github,
                            label: "Open source",
                        }

                        TrustItem {
                            icon: IconKind::Heart,
                            label: "Loved by developers",
                        }
                    }
                }

                div {
                    class: "relative",

                    div {
                        class: "absolute -inset-12 -z-10 rounded-full bg-violet-500/10 blur-3xl",
                    }

                    div {
                        class: "flex min-h-[360px] items-center justify-center rounded-3xl border border-white/10 bg-white/[0.035] p-3 shadow-2xl shadow-black/30 backdrop-blur",

                        span {
                            class: "text-sm font-medium uppercase tracking-[0.22em] text-[color:var(--text-muted)]",
                            "No preview yet"
                        }
                    }
                }
            }
        }
    }
}
