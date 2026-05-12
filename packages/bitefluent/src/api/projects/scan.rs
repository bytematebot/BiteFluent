use super::dto::{FluentDirectoryDto, FluentFileDto, ProjectScanDto};
use dioxus::prelude::dioxus_fullstack::HeaderMap;
use dioxus::prelude::*;

#[post("/api/projects/:project_id/scan-fluent", headers: HeaderMap)]
pub async fn scan_project_fluent_files(project_id: String) -> ServerFnResult<ProjectScanDto> {
    #[cfg(feature = "server")]
    {
        use super::helpers::session_cookie;
        use crate::auth::{BiteFluentAuthAdapter, BiteFluentSessionStore, Db};
        use crate::integrations::github::GithubClient;
        use dioxus_auth::{AuthAdapter, SessionStore};

        dotenvy::dotenv().ok();

        let Some(session_token) = session_cookie(&headers, "bitefluent.session") else {
            return Err(ServerFnError::new("unauthorized"));
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
            return Err(ServerFnError::new("unauthorized"));
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
            return Err(ServerFnError::new("project not found"));
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

        let tree = GithubClient::new(access_token)
            .repository_tree(
                &project.repository_owner,
                &project.repository_name,
                &project.default_branch,
            )
            .await
            .map_err(|error| ServerFnError::new(error.to_string()))?;

        let files = tree
            .into_iter()
            .filter(|item| item.kind == "blob")
            .filter(|item| item.path.ends_with(".ftl"))
            .map(|item| FluentFileDto {
                locale: infer_locale_from_path(&item.path),
                path: item.path,
            })
            .collect::<Vec<_>>();

        let directories = detect_fluent_directories(&files);
        let suggested_locales_path = directories
            .iter()
            .find(|directory| directory.recommended)
            .map(|directory| directory.path.clone())
            .or_else(|| suggest_locales_path(&files));

        let scoped_files = suggested_locales_path
            .as_ref()
            .map(|path| files_in_directory(&files, path))
            .unwrap_or_else(|| files.clone());

        let suggested_source_locale =
            suggest_source_locale(&scoped_files).or_else(|| suggest_source_locale(&files));

        Ok(ProjectScanDto {
            project_id: project.id,
            repository_full_name: project.repository_full_name,
            files,
            directories,
            suggested_locales_path,
            suggested_source_locale,
        })
    }

    #[cfg(not(feature = "server"))]
    {
        Err(ServerFnError::new(
            "scan_project_fluent_files can only run on the server",
        ))
    }
}

#[cfg(feature = "server")]
fn detect_fluent_directories(files: &[FluentFileDto]) -> Vec<FluentDirectoryDto> {
    use std::collections::{BTreeMap, BTreeSet};

    let mut grouped = BTreeMap::<String, Vec<&FluentFileDto>>::new();

    for file in files {
        let directory = fluent_directory_for_path(&file.path, file.locale.as_deref());
        grouped.entry(directory).or_default().push(file);
    }

    let mut directories = grouped
        .into_iter()
        .map(|(path, files)| {
            let locales = files
                .iter()
                .filter_map(|file| file.locale.clone())
                .collect::<BTreeSet<_>>()
                .into_iter()
                .collect::<Vec<_>>();

            let files_count = files.len();
            let score = score_fluent_directory(&path, files_count, locales.len());

            FluentDirectoryDto {
                path,
                files_count,
                locales,
                recommended: false,
                score,
            }
        })
        .collect::<Vec<_>>();

    directories.sort_by(|left, right| {
        right
            .score
            .cmp(&left.score)
            .then_with(|| right.files_count.cmp(&left.files_count))
            .then_with(|| right.locales.len().cmp(&left.locales.len()))
            .then_with(|| left.path.cmp(&right.path))
    });

    if let Some(first) = directories.first_mut() {
        first.recommended = true;
    }

    directories
}

#[cfg(feature = "server")]
fn fluent_directory_for_path(path: &str, locale: Option<&str>) -> String {
    let parts = path.split('/').collect::<Vec<_>>();

    if let Some(locale) = locale {
        if let Some(index) = parts
            .iter()
            .position(|part| normalize_locale(part) == normalize_locale(locale))
        {
            let directory = parts[..index].join("/");
            return normalize_directory_path(directory);
        }
    }

    let directory = path
        .rsplit_once('/')
        .map(|(directory, _)| directory.to_string())
        .unwrap_or_else(|| ".".to_string());

    normalize_directory_path(directory)
}

#[cfg(feature = "server")]
fn normalize_directory_path(path: String) -> String {
    let path = path.trim_matches('/').trim().to_string();

    if path.is_empty() {
        ".".to_string()
    } else {
        path
    }
}

#[cfg(feature = "server")]
fn score_fluent_directory(path: &str, files_count: usize, locales_count: usize) -> i32 {
    let lower = path.to_lowercase();
    let parts = lower.split('/').collect::<Vec<_>>();

    let mut score = 0;

    score += files_count as i32 * 4;
    score += locales_count as i32 * 12;

    for part in parts {
        match part {
            "locales" | "locale" | "translations" | "translation" | "i18n" | "l10n" => {
                score += 40;
            }
            "examples" | "example" | "demo" | "demos" | "test" | "tests" | "fixtures"
            | "fixture" | "samples" | "sample" => {
                score -= 80;
            }
            "node_modules" | "target" | "dist" | "build" | "vendor" => {
                score -= 120;
            }
            _ => {}
        }
    }

    score
}

#[cfg(feature = "server")]
fn files_in_directory(files: &[FluentFileDto], directory: &str) -> Vec<FluentFileDto> {
    if directory == "." {
        return files.to_vec();
    }

    let prefix = format!("{directory}/");

    files
        .iter()
        .filter(|file| file.path.starts_with(&prefix))
        .cloned()
        .collect()
}

#[cfg(feature = "server")]
fn infer_locale_from_path(path: &str) -> Option<String> {
    let parts = path.split('/').collect::<Vec<_>>();

    for part in &parts {
        if looks_like_locale(part) {
            return Some(normalize_locale(part));
        }
    }

    let file_name = parts.last()?;
    let stem = file_name.strip_suffix(".ftl").unwrap_or(file_name);

    if looks_like_locale(stem) {
        return Some(normalize_locale(stem));
    }

    None
}

#[cfg(feature = "server")]
fn looks_like_locale(value: &str) -> bool {
    let normalized = normalize_locale(value);
    let parts = normalized.split('-').collect::<Vec<_>>();

    match parts.as_slice() {
        [language] => language.len() == 2 || language.len() == 3,
        [language, region] => {
            (language.len() == 2 || language.len() == 3) && (region.len() == 2 || region.len() == 4)
        }
        _ => false,
    }
}

#[cfg(feature = "server")]
fn normalize_locale(value: &str) -> String {
    value.replace('_', "-")
}

#[cfg(feature = "server")]
fn suggest_source_locale(files: &[FluentFileDto]) -> Option<String> {
    let preferred = ["en-US", "en", "en-GB"];

    for locale in preferred {
        if files
            .iter()
            .any(|file| file.locale.as_deref() == Some(locale))
        {
            return Some(locale.to_string());
        }
    }

    files.iter().find_map(|file| file.locale.clone())
}

#[cfg(feature = "server")]
fn suggest_locales_path(files: &[FluentFileDto]) -> Option<String> {
    let first = files.first()?;
    Some(fluent_directory_for_path(
        &first.path,
        first.locale.as_deref(),
    ))
}
