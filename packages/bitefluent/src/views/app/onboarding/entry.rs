use crate::api::auth::AuthUserDto;
use crate::api::projects::fetch_current_onboarding_project;
use crate::components::MetaFor;
use crate::views::app::loading::{AppLoading, AppRedirecting};
use crate::views::app::onboarding::Onboarding;
use dioxus::prelude::*;

#[component]
pub fn AppOnboardingEntry(user: AuthUserDto) -> Element {
    let navigator = use_navigator();
    let onboarding_project = use_resource(fetch_current_onboarding_project);
    let project_state = onboarding_project.read().clone();

    use_effect({
        let project_state = project_state.clone();

        move || {
            if let Some(Ok(Some(project))) = project_state.as_ref() {
                navigator.replace(format!("/app/onboarding/{}", project.id));
            }
        }
    });

    match project_state {
        None => rsx! {
            AppLoading {}
        },

        Some(Err(_)) => rsx! {
            crate::components::Meta { r#for: MetaFor::Home }

            Onboarding {
                user,
            }
        },

        Some(Ok(Some(_project))) => rsx! {
            AppRedirecting {}
        },

        Some(Ok(None)) => rsx! {
            crate::components::Meta { r#for: MetaFor::Home }

            Onboarding {
                user,
            }
        },
    }
}
