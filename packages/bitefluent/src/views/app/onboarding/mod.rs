mod entry;
mod helpers;
mod intro;
mod project;
mod repository_card;
mod repository_row;

pub use entry::AppOnboardingEntry;
pub use project::OnboardingProject;

use crate::api::auth::AuthUserDto;
use crate::api::github::fetch_github_repositories;
use dioxus::prelude::*;
use intro::OnboardingIntro;
use repository_card::RepositoryCard;

#[component]
pub fn Onboarding(user: AuthUserDto) -> Element {
    let mut repositories = use_resource(fetch_github_repositories);
    let selected_repository_id = use_signal(|| None::<u64>);
    let selected_owner = use_signal(|| None::<String>);
    let repository_state = repositories.read().clone();

    rsx! {
        section {
            class: "mx-auto grid min-h-screen w-full max-w-7xl items-center gap-16 px-6 py-28 lg:grid-cols-[0.85fr_1.15fr] lg:px-10",

            OnboardingIntro {}

            RepositoryCard {
                user,
                repository_state,
                selected_repository_id,
                selected_owner,
                on_reload: move |_| repositories.restart(),
            }
        }
    }
}
