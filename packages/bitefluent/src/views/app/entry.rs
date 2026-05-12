use crate::api::auth::{AuthUserDto, AuthUserResource, AuthUserState, read_auth_user};
use crate::views::app::dashboard::AppDashboard;
use crate::views::app::loading::{AppLoading, AppRedirecting};
use crate::views::app::onboarding::{AppOnboardingEntry, OnboardingProject};
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
            AppEntry {
                user,
            }
        },
    }
}

#[component]
fn AppEntry(user: AuthUserDto) -> Element {
    if user.onboarded {
        return rsx! {
            AppDashboard {
                user,
            }
        };
    }

    rsx! {
        AppOnboardingEntry {
            user,
        }
    }
}

#[component]
pub fn AppOnboardingProject(project_id: String) -> Element {
    rsx! {
        OnboardingProject {
            project_id,
        }
    }
}
