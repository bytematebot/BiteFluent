use super::dto::{ProjectWorkspaceDto, ProjectWorkspaceFileDto, ProjectWorkspaceKeyDto};
use dioxus::prelude::dioxus_fullstack::HeaderMap;
use dioxus::prelude::*;

#[get("/api/projects/:project_id/workspace", headers: HeaderMap)]
pub async fn fetch_project_workspace(
    project_id: String,
) -> ServerFnResult<Option<ProjectWorkspaceDto>> {
    #[cfg(feature = "server")]
    {
        use super::dto::map_project;
        use super::helpers::session_cookie;
        use crate::auth::{BiteFluentAuthAdapter, BiteFluentSessionStore, Db};
        use crate::integrations::github::GithubClient;
        use dioxus_auth::{AuthAdapter, SessionStore};

        dotenvy::dotenv().ok();

        let Some(session_token) = session_cookie(&headers, "bitefluent.session") else {
            return Ok(None);
        };

        let database_url =
            std::env::var("DATABASE_URL").map_err(|error| ServerFnError::new(error.to_string()))?;

        let client = byteorm_client::Client::new(&database_url)
            .await
            .map_err(|error| ServerFnError::new(error.to_string()))?;

        let db = Db::new(client);
        let session_store = BiteFluentSessionStore::new(db.clone());

        let Some(session) = session_store
            .get_session(&session_token)
            .await
            .map_err(|error| ServerFnError::new(error.to_string()))?
        else {
            return Ok(None);
        };

        let Some(project) = db
            .client
            .projects
            .find_first(|project| {
                project
                    .where_id(project_id.clone())
                    .where_owner_id(session.user_id.clone())
            })
            .await
            .map_err(|error| ServerFnError::new(error.to_string()))?
        else {
            return Ok(None);
        };

        let adapter = BiteFluentAuthAdapter::new(db.clone());

        let Some(account) = adapter
            .get_github_account_for_user(&session.user_id)
            .await
            .map_err(|error| ServerFnError::new(error.to_string()))?
        else {
            return Err(ServerFnError::new("missing GitHub account"));
        };

        let Some(access_token) = account.access_token else {
            return Err(ServerFnError::new("missing GitHub access token"));
        };

        let github = GithubClient::new(access_token);

        let locales_path = project
            .locales_path
            .clone()
            .unwrap_or_else(|| ".".to_string());

        let source_locale = project.source_locale.clone();

        let tree = github
            .repository_tree_under_path(
                &project.repository_owner,
                &project.repository_name,
                &project.default_branch,
                &locales_path,
            )
            .await
            .map_err(|error| ServerFnError::new(error.to_string()))?;

        let mut files = tree
            .into_iter()
            .filter(|item| item.kind == "blob")
            .filter(|item| item.path.ends_with(".ftl"))
            .map(|item| ProjectWorkspaceFileDto {
                name: item
                    .path
                    .rsplit('/')
                    .next()
                    .unwrap_or(&item.path)
                    .to_string(),
                locale: infer_locale_from_project_path(&item.path, source_locale.as_deref()),
                path: item.path,
                sha: item.sha,
            })
            .collect::<Vec<_>>();

        files.sort_by(|left, right| {
            left.locale
                .cmp(&right.locale)
                .then_with(|| left.path.cmp(&right.path))
        });

        let mut keys = Vec::new();

        for file in files.iter() {
            let content = github
                .repository_blob_content(
                    &project.repository_owner,
                    &project.repository_name,
                    &file.sha,
                )
                .await
                .map_err(|error| ServerFnError::new(error.to_string()))?;

            keys.extend(parse_fluent_keys(&file.path, file.locale.clone(), &content));
        }

        keys.sort_by(|left, right| {
            left.file_path
                .cmp(&right.file_path)
                .then_with(|| left.key.cmp(&right.key))
        });

        Ok(Some(ProjectWorkspaceDto {
            project: map_project(project),
            files,
            keys,
        }))
    }

    #[cfg(not(feature = "server"))]
    {
        Err(ServerFnError::new(
            "fetch_project_workspace can only run on the server",
        ))
    }
}

#[cfg(feature = "server")]
fn parse_fluent_keys(
    file_path: &str,
    locale: Option<String>,
    content: &str,
) -> Vec<ProjectWorkspaceKeyDto> {
    let mut keys = Vec::new();
    let mut current_key: Option<String> = None;
    let mut current_value = String::new();

    for line in content.lines() {
        let trimmed = line.trim();

        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }

        if !line.starts_with(' ') && !line.starts_with('\t') {
            if let Some(key) = current_key.take() {
                keys.push(ProjectWorkspaceKeyDto {
                    key,
                    file_path: file_path.to_string(),
                    locale: locale.clone(),
                    value: current_value.trim().to_string(),
                });

                current_value.clear();
            }

            if let Some((key, value)) = trimmed.split_once('=') {
                current_key = Some(key.trim().to_string());
                current_value = value.trim().to_string();
            }
        } else if current_key.is_some() {
            if !current_value.is_empty() {
                current_value.push('\n');
            }

            current_value.push_str(trimmed);
        }
    }

    if let Some(key) = current_key {
        keys.push(ProjectWorkspaceKeyDto {
            key,
            file_path: file_path.to_string(),
            locale,
            value: current_value.trim().to_string(),
        });
    }

    keys
}

#[cfg(feature = "server")]
fn infer_locale_from_project_path(path: &str, source_locale: Option<&str>) -> Option<String> {
    for part in path.split('/') {
        if looks_like_locale(part) {
            return Some(part.replace('_', "-"));
        }
    }

    source_locale.map(ToString::to_string)
}

#[cfg(feature = "server")]
fn looks_like_locale(value: &str) -> bool {
    let normalized = value.replace('_', "-");
    let parts = normalized.split('-').collect::<Vec<_>>();

    match parts.as_slice() {
        [language] => language.len() == 2 || language.len() == 3,
        [language, region] => {
            (language.len() == 2 || language.len() == 3)
                && (region.len() == 2 || region.len() == 4)
        }
        _ => false,
    }
}