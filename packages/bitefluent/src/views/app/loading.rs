use dioxus::prelude::*;

#[component]
pub fn AppLoading() -> Element {
    rsx! {
        section {
            class: "mx-auto flex min-h-screen w-full max-w-7xl items-center px-6 py-24 lg:px-10",

            div {
                class: "h-28 w-full max-w-xl animate-pulse rounded-3xl bg-white/[0.04]",
            }
        }
    }
}

#[component]
pub fn AppRedirecting() -> Element {
    rsx! {
        section {
            class: "mx-auto flex min-h-screen w-full max-w-7xl items-center px-6 py-24 lg:px-10",

            p {
                class: "text-sm text-[color:var(--text-muted)]",
                "Redirecting..."
            }
        }
    }
}
