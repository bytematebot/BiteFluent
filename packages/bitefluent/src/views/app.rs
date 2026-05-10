use crate::components::MetaFor;
use dioxus::prelude::*;

#[component]
pub fn AppPlatform() -> Element {
    rsx! {
        crate::components::Meta { r#for: MetaFor::Home }

        section {
            class: "mx-auto flex min-h-screen w-full max-w-7xl flex-col justify-center px-6 py-24 lg:px-10",

            div {
                class: "max-w-3xl",

                p {
                    class: "text-sm font-medium uppercase tracking-[0.2em] text-[color:var(--text-secondary)]",
                    "Platform"
                }

                h1 {
                    class: "mt-4 font-serif text-5xl leading-[0.95] tracking-[-0.04em] text-[color:var(--text-primary)] sm:text-6xl lg:text-7xl",
                    "BiteFluent"
                }

                p {
                    class: "mt-6 max-w-2xl text-lg leading-8 text-[color:var(--text-secondary)]",
                    "You are signed in. This is the workspace entry point."
                }
            }
        }
    }
}
