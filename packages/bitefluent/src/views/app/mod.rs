pub mod onboarding;

use crate::api::auth::{read_auth_user, AuthUserResource, AuthUserState};
use crate::components::MetaFor;
use dioxus::prelude::*;

#[component]
pub fn AppPlatform() -> Element {
    let navigator = use_navigator();

    let current_user = use_context::<AuthUserResource>();
    let auth_state = read_auth_user(&current_user);

    use_effect({
        let auth_state = auth_state.clone();

        move || {
            if auth_state.is_guest() {
                navigator.replace("/");
            }
        }
    });

    match auth_state {
        AuthUserState::Loading => rsx! {
            AppLoading {}
        },

        AuthUserState::Guest => rsx! {
            AppRedirecting {}
        },

        AuthUserState::Authenticated(user) => rsx! {
            crate::components::Meta { r#for: MetaFor::Home }

            onboarding::Onboarding {
                user,
            }
        },
    }
}

#[component]
fn AppLoading() -> Element {
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
fn AppRedirecting() -> Element {
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