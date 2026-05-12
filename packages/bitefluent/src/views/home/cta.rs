use dioxus::prelude::*;

use crate::components::{
    Button, ButtonSize, ButtonVariant, Icon, IconKind, IconPosition, RenderIcon,
};

#[component]
pub fn CtaSection() -> Element {
    rsx! {
        section {
            class: "mx-auto w-full max-w-7xl px-6 py-24 lg:px-10",

            div {
                class: "relative overflow-hidden rounded-3xl border border-white/[0.08] bg-white/[0.03] px-6 py-10 shadow-2xl shadow-black/20 sm:px-10 lg:px-14",

                div {
                    class: "pointer-events-none absolute inset-y-0 right-0 w-1/2 bg-violet-500/20 blur-3xl",
                }

                div {
                    class: "relative grid gap-10 lg:grid-cols-[1fr_1.2fr] lg:items-center",

                    div {
                        h2 {
                            class: "max-w-md font-serif text-3xl leading-[0.95] tracking-[-0.04em] text-[color:var(--text-primary)] sm:text-4xl",
                            "Ready to ship better translations?"
                        }

                        div {
                            class: "mt-7 flex flex-wrap items-center gap-x-5 gap-y-3",

                            CtaBenefit {
                                label: "Free to get started",
                            }

                            CtaBenefit {
                                label: "Open source",
                            }

                            CtaBenefit {
                                label: "No credit card required",
                            }
                        }
                    }

                    div {
                        class: "lg:justify-self-end",

                        p {
                            class: "max-w-xl text-sm leading-6 text-[color:var(--text-secondary)]",
                            "Join developers and localization teams who" br {} "trust BiteFluent to manage translations at scale."
                        }

                        div {
                            class: "mt-7 flex flex-col gap-4 sm:flex-row",

                            Button {
                                label: "Get Started for Free",
                                variant: ButtonVariant::Primary,
                                size: ButtonSize::Md,
                                icon: Some(Icon {
                                    kind: IconKind::Arrow,
                                    position: IconPosition::Right,
                                    class: Some("size-4".to_string()),
                                }),
                                to: "/start",
                            }

                            Button {
                                label: "View on GitHub",
                                variant: ButtonVariant::Secondary,
                                href: "https://github.com/bytematebot/bitefluent",
                                size: ButtonSize::Md,
                                icon: Some(Icon {
                                    kind: IconKind::Github,
                                    position: IconPosition::Left,
                                    class: Some("size-4".to_string()),
                                }),
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn CtaBenefit(label: &'static str) -> Element {
    rsx! {
        div {
            class: "inline-flex items-center gap-2 text-xs font-medium leading-none text-[color:var(--text-muted)]",

            span {
                class: "flex size-3.5 shrink-0 items-center justify-center text-[color:var(--bg-accent)]",

                RenderIcon {
                    kind: IconKind::CheckNoCircle,
                    class: Some("size-3.5".to_string()),
                }
            }

            span {
                class: "leading-none",
                "{label}"
            }
        }
    }
}
