use crate::api::github::GithubRepositoryDto;
use dioxus::prelude::*;

pub fn repository_owners(repositories: &[GithubRepositoryDto]) -> Vec<String> {
    let mut owners = repositories
        .iter()
        .map(|repository| repository.owner.clone())
        .collect::<Vec<_>>();

    owners.sort();
    owners.dedup();

    owners
}

pub fn effective_owner(
    repository_state: &Option<ServerFnResult<Vec<GithubRepositoryDto>>>,
    selected_owner: Signal<Option<String>>,
    fallback: String,
) -> String {
    selected_owner().unwrap_or_else(|| {
        repository_state
            .as_ref()
            .and_then(|state| state.as_ref().ok())
            .map(|repositories| repository_owners(repositories))
            .and_then(|owners| owners.first().cloned())
            .unwrap_or(fallback)
    })
}

pub fn effective_repository_id(
    selected_id: Option<u64>,
    visible_repositories: &[GithubRepositoryDto],
) -> Option<u64> {
    selected_id
        .filter(|id| visible_repositories.iter().any(|repo| repo.id == *id))
        .or_else(|| visible_repositories.first().map(|repo| repo.id))
}

pub fn placeholder_repository(id: u64) -> GithubRepositoryDto {
    GithubRepositoryDto {
        id,
        owner: "owner".to_string(),
        name: "repository".to_string(),
        full_name: "owner/repository".to_string(),
        private: true,
        default_branch: "main".to_string(),
    }
}