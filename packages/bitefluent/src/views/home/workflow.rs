use dioxus::prelude::*;

use crate::components::{IconKind, RenderIcon};

#[component]
pub fn WorkflowSection() -> Element {
    rsx! {
        section {
            id: "workflow",
            class: "mx-auto w-full max-w-7xl px-6 py-24 lg:px-10",

            div {
                class: "rounded-3xl border border-white/[0.08] bg-white/[0.025] px-6 py-16 shadow-2xl shadow-black/20 sm:px-10 lg:px-16",

                div {
                    class: "mx-auto max-w-2xl text-center",

                    p {
                        class: "text-xs font-bold uppercase tracking-[0.22em] text-[color:var(--bg-accent)]",
                        "Simple workflow"
                    }

                    h2 {
                        class: "mt-4 font-serif text-3xl leading-[0.95] tracking-[-0.04em] text-[color:var(--text-primary)] sm:text-4xl",
                        "From source to localized—streamlined."
                    }
                }

                div {
                    class: "relative mt-14 grid gap-12 lg:grid-cols-3 lg:gap-8",

                    div {
                        class: "pointer-events-none absolute left-[16.666%] right-[16.666%] top-5 hidden border-t border-dashed border-white/[0.12] lg:block",
                    }

                    WorkflowStep {
                        number: "1",
                        icon: IconKind::Github,
                        title: "Connect your repo",
                        description: "Connect GitHub and select your Fluent project. BiteFluent analyzes your files and sets everything up.",
                    }

                    WorkflowStep {
                        number: "2",
                        icon: IconKind::File,
                        title: "Sync source messages",
                        description: "We pull new and changed keys, detect outdated translations, and keep your locales in sync.",
                    }

                    WorkflowStep {
                        number: "3",
                        icon: IconKind::Check,
                        title: "Translate & review",
                        description: "Translate with context, review changes, and publish. We’ll create a PR so you can ship with confidence.",
                    }
                }
            }
        }
    }
}

#[component]
fn WorkflowStep(
    number: &'static str,
    icon: IconKind,
    title: &'static str,
    description: &'static str,
) -> Element {
    rsx! {
        div {
            class: "relative flex flex-col items-center text-center",

            div {
                class: "relative z-10 flex size-10 items-center justify-center rounded-full border border-[color:var(--bg-accent)]/30 bg-[#121218] text-sm font-semibold text-[color:var(--text-primary)]",
                "{number}"
            }

            div {
                class: "mt-8 flex size-9 items-center justify-center text-[color:var(--text-muted)]",

                RenderIcon {
                    kind: icon,
                    class: Some("size-7".to_string()),
                }
            }

            h3 {
                class: "mt-5 font-serif text-xl leading-none tracking-[-0.03em] text-[color:var(--text-primary)]",
                "{title}"
            }

            p {
                class: "mt-4 max-w-xs text-sm leading-6 text-[color:var(--text-muted)]",
                "{description}"
            }
        }
    }
}
