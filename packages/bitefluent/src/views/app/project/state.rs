use crate::components::Skeleton;
use dioxus::prelude::*;

#[component]
pub fn ProjectSkeleton() -> Element {
    rsx! {
        section {
            class: "mx-auto flex min-h-screen w-full max-w-[1800px] flex-col px-4 pt-24 pb-4 lg:px-6",

            Skeleton {
                suspend: true,
                class: "h-12 w-96 rounded-xl bg-white/[0.06]".to_string(),
                div {}
            }

            div {
                class: "mt-5 grid min-h-[calc(100vh-9rem)] overflow-hidden rounded-3xl border border-white/[0.10] bg-white/[0.025] lg:grid-cols-[18rem_minmax(22rem,30rem)_1fr]",

                Skeleton {
                    suspend: true,
                    class: "h-full min-h-[40rem] rounded-none border-r border-white/[0.08] bg-white/[0.02]".to_string(),
                    div {}
                }

                Skeleton {
                    suspend: true,
                    class: "h-full min-h-[40rem] rounded-none border-r border-white/[0.08] bg-white/[0.02]".to_string(),
                    div {}
                }

                Skeleton {
                    suspend: true,
                    class: "h-full min-h-[40rem] rounded-none bg-white/[0.02]".to_string(),
                    div {}
                }
            }
        }
    }
}

#[component]
pub fn ProjectNotFound() -> Element {
    rsx! {
        section {
            class: "mx-auto flex min-h-screen w-full max-w-7xl flex-col px-6 pt-36 pb-24 lg:px-10",

            div {
                class: "max-w-3xl",

                h1 {
                    class: "font-serif text-5xl leading-[0.95] tracking-[-0.04em] text-[color:var(--text-primary)]",
                    "Project not found."
                }

                p {
                    class: "mt-6 text-lg leading-8 text-[color:var(--text-secondary)]",
                    "This project does not exist or you do not have access to it."
                }
            }
        }
    }
}
