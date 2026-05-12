use super::{ProjectSyncDto, ProjectSyncProgressDto};
use dioxus::fullstack::{JsonEncoding, Streaming};
use dioxus::prelude::dioxus_fullstack::HeaderMap;
use dioxus::prelude::*;
use std::collections::{BTreeMap, BTreeSet};

#[post("/api/projects/:project_id/sync", headers: HeaderMap)]
pub async fn sync_project_from_github(project_id: String) -> ServerFnResult<ProjectSyncDto> {
    #[cfg(feature = "server")]
    {
        sync_project_from_github_inner(project_id, headers, |_| {}).await
    }

    #[cfg(not(feature = "server"))]
    {
        Err(ServerFnError::new(
            "sync_project_from_github can only run on the server",
        ))
    }
}

#[get("/api/projects/:project_id/sync-stream", headers: HeaderMap)]
pub async fn sync_project_from_github_stream(
    project_id: String,
) -> ServerFnResult<Streaming<ProjectSyncProgressDto, JsonEncoding>> {
    #[cfg(feature = "server")]
    {
        Ok(Streaming::spawn(move |tx| async move {
            let send = |progress: ProjectSyncProgressDto| {
                let _ = tx.unbounded_send(progress);
            };

            send(ProjectSyncProgressDto {
                total_files: 0,
                processed_files: 0,
                keys_count: 0,
                translations_count: 0,
                locales_count: 0,
                message: "Starting sync...".to_string(),
                done: false,
                error: None,
            });

            let result = sync_project_from_github_inner(project_id, headers, send).await;

            if let Err(error) = result {
                let _ = tx.unbounded_send(ProjectSyncProgressDto {
                    total_files: 0,
                    processed_files: 0,
                    keys_count: 0,
                    translations_count: 0,
                    locales_count: 0,
                    message: "Sync failed".to_string(),
                    done: true,
                    error: Some(error.to_string()),
                });
            }
        }))
    }

    #[cfg(not(feature = "server"))]
    {
        Err(ServerFnError::new(
            "sync_project_from_github_stream can only run on the server",
        ))
    }
}

#[cfg(feature = "server")]
async fn sync_project_from_github_inner(
    project_id: String,
    headers: HeaderMap,
    mut progress: impl FnMut(ProjectSyncProgressDto),
) -> ServerFnResult<ProjectSyncDto> {
    use super::helpers::session_cookie;
    use crate::auth::{BiteFluentAuthAdapter, BiteFluentSessionStore, Db};
    use crate::integrations::github::GithubClient;
    use dioxus_auth::{AuthAdapter, SessionStore};
    use futures_util::stream::{self, StreamExt};
    use std::collections::{BTreeMap, BTreeSet};
    use time::OffsetDateTime;

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

    let Some(source_locale) = project.source_locale.clone() else {
        return Err(ServerFnError::new("project source locale is not set"));
    };

    let locales_path = project
        .locales_path
        .clone()
        .unwrap_or_else(|| ".".to_string());

    let now = OffsetDateTime::now_utc();
    let now_chrono =
        crate::auth::time_to_chrono(now).map_err(|error| ServerFnError::new(error.to_string()))?;

    db.client
        .projects
        .update(|project_update| {
            project_update
                .where_id(project.id.clone())
                .where_owner_id(session.user_id.clone())
                .set_sync_status(Some("syncing".to_string()))
                .set_sync_error(None)
                .set_updated_at(now_chrono)
        })
        .await
        .map_err(|error| ServerFnError::new(error.to_string()))?;

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

    progress(ProjectSyncProgressDto {
        total_files: 0,
        processed_files: 0,
        keys_count: 0,
        translations_count: 0,
        locales_count: 0,
        message: "Scanning repository...".to_string(),
        done: false,
        error: None,
    });

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
        .map(|item| SyncFile {
            path: item.path.clone(),
            sha: item.sha,
            locale: infer_locale_from_project_path(&item.path, Some(&source_locale)),
        })
        .collect::<Vec<_>>();

    files.sort_by(|left, right| {
        left.locale
            .cmp(&right.locale)
            .then_with(|| left.path.cmp(&right.path))
    });

    let total_files = files.len();

    let mut locales = BTreeSet::<String>::new();

    for file in files.iter() {
        if let Some(locale) = file.locale.as_ref() {
            locales.insert(locale.clone());
        }
    }

    let existing_file_shas = fetch_existing_file_shas(&db, &project.id).await?;

    let current_file_paths = files
        .iter()
        .map(|file| file.path.clone())
        .collect::<BTreeSet<_>>();

    let deleted_file_paths = existing_file_shas
        .keys()
        .filter(|path| !current_file_paths.contains(*path))
        .cloned()
        .collect::<Vec<_>>();

    let changed_files = files
        .iter()
        .filter(|file| {
            existing_file_shas
                .get(&file.path)
                .map(|existing_sha| existing_sha != &file.sha)
                .unwrap_or(true)
        })
        .cloned()
        .collect::<Vec<_>>();

    let total_changed_files = changed_files.len();
    let total_deleted_files = deleted_file_paths.len();
    let total_work_files = total_changed_files + total_deleted_files;

    progress(ProjectSyncProgressDto {
        total_files: total_work_files,
        processed_files: 0,
        keys_count: 0,
        translations_count: 0,
        locales_count: locales.len(),
        message: match (total_changed_files, total_deleted_files) {
            (0, 0) => format!("Found {total_files} Fluent files, no changes"),
            (_, 0) => format!("Found {total_files} Fluent files, {total_changed_files} changed"),
            (0, _) => format!("Found {total_files} Fluent files, {total_deleted_files} deleted"),
            _ => format!(
                "Found {total_files} Fluent files, {total_changed_files} changed, {total_deleted_files} deleted"
            ),
        },
        done: false,
        error: None,
    });

    let owner = project.repository_owner.clone();
    let repo = project.repository_name.clone();

    let mut parsed_by_locale = BTreeMap::<String, Vec<ParsedFluentEntry>>::new();
    let mut processed_files = 0usize;
    let mut parsed_keys_count = 0usize;

    let mut blob_stream = stream::iter(changed_files.clone())
        .map(|file| {
            let github = github.clone();
            let owner = owner.clone();
            let repo = repo.clone();

            async move {
                let entries = if file.locale.is_some() {
                    let content = github
                        .repository_blob_content(&owner, &repo, &file.sha)
                        .await?;

                    parse_fluent_entries(&file.path, &content)
                } else {
                    Vec::new()
                };

                anyhow::Ok((file, entries))
            }
        })
        .buffer_unordered(8);

    while let Some(result) = blob_stream.next().await {
        let (file, entries) = result.map_err(|error| ServerFnError::new(error.to_string()))?;

        processed_files += 1;

        let message = if let Some(locale) = file.locale.clone() {
            parsed_keys_count += entries.len();

            parsed_by_locale.entry(locale).or_default().extend(entries);

            format!("Synced {}", file.path)
        } else {
            format!("Skipped {}", file.path)
        };

        progress(ProjectSyncProgressDto {
            total_files: total_work_files,
            processed_files,
            keys_count: parsed_keys_count,
            translations_count: 0,
            locales_count: locales.len(),
            message,
            done: false,
            error: None,
        });
    }

    if !changed_files.is_empty() {
        progress(ProjectSyncProgressDto {
            total_files: total_work_files,
            processed_files,
            keys_count: parsed_keys_count,
            translations_count: 0,
            locales_count: locales.len(),
            message: "Saving file manifest...".to_string(),
            done: false,
            error: None,
        });

        upsert_project_files_many(&db, &project.id, &changed_files, now_chrono).await?;
    }

    let source_entries = parsed_by_locale
        .get(&source_locale)
        .map(|entries| dedupe_entries_by_key(entries))
        .unwrap_or_default();

    if !source_entries.is_empty() {
        progress(ProjectSyncProgressDto {
            total_files: total_work_files,
            processed_files,
            keys_count: source_entries.len(),
            translations_count: 0,
            locales_count: locales.len(),
            message: "Saving source keys...".to_string(),
            done: false,
            error: None,
        });

        upsert_translation_keys_many(&db, &project.id, &source_entries, now_chrono).await?;
    }

    let changed_source_file_paths = changed_files
        .iter()
        .filter(|file| file.locale.as_deref() == Some(source_locale.as_str()))
        .map(|file| file.path.clone())
        .collect::<Vec<_>>();

    if !changed_source_file_paths.is_empty() {
        let source_keys_by_file = source_keys_by_file(&source_entries);

        cleanup_missing_source_keys_in_changed_files(
            &db,
            &project.id,
            &changed_source_file_paths,
            &source_keys_by_file,
        )
        .await?;
    }

    let key_ids_by_key = if parsed_by_locale
        .keys()
        .any(|locale| locale != &source_locale)
    {
        fetch_translation_key_ids(&db, &project.id).await?
    } else {
        BTreeMap::new()
    };

    let mut translation_values = Vec::<SyncTranslationValue>::new();

    for (locale, entries) in parsed_by_locale.iter() {
        if locale == &source_locale {
            continue;
        }

        let entries = dedupe_entries_by_key(entries);

        for entry in entries {
            let Some(key_id) = key_ids_by_key.get(&entry.key) else {
                continue;
            };

            translation_values.push(SyncTranslationValue {
                key_id: key_id.clone(),
                locale: locale.clone(),
                file_path: entry.file_path,
                value: entry.value,
            });
        }
    }

    if !translation_values.is_empty() {
        progress(ProjectSyncProgressDto {
            total_files: total_work_files,
            processed_files,
            keys_count: source_entries.len(),
            translations_count: translation_values.len(),
            locales_count: locales.len(),
            message: "Saving translations...".to_string(),
            done: false,
            error: None,
        });

        upsert_translation_values_many(&db, &project.id, &translation_values, now_chrono).await?;
    }

    if !deleted_file_paths.is_empty() {
        progress(ProjectSyncProgressDto {
            total_files: total_work_files,
            processed_files: total_changed_files,
            keys_count: source_entries.len(),
            translations_count: translation_values.len(),
            locales_count: locales.len(),
            message: format!("Cleaning up {} deleted files...", deleted_file_paths.len()),
            done: false,
            error: None,
        });

        cleanup_deleted_files(&db, &project.id, &source_locale, &deleted_file_paths).await?;
    }

    let finished_at = OffsetDateTime::now_utc();
    let finished_at_chrono = crate::auth::time_to_chrono(finished_at)
        .map_err(|error| ServerFnError::new(error.to_string()))?;

    db.client
        .projects
        .update(|project_update| {
            project_update
                .where_id(project.id.clone())
                .where_owner_id(session.user_id.clone())
                .set_last_synced_at(Some(finished_at_chrono))
                .set_sync_status(Some("synced".to_string()))
                .set_sync_error(None)
                .set_updated_at(finished_at_chrono)
        })
        .await
        .map_err(|error| ServerFnError::new(error.to_string()))?;

    let result = ProjectSyncDto {
        files_count: total_files,
        keys_count: source_entries.len(),
        translations_count: translation_values.len(),
        locales_count: locales.len(),
        last_synced_at: Some(finished_at_chrono.to_rfc3339()),
        sync_status: Some("synced".to_string()),
        sync_error: None,
    };

    progress(ProjectSyncProgressDto {
        total_files: total_work_files,
        processed_files: total_work_files,
        keys_count: result.keys_count,
        translations_count: result.translations_count,
        locales_count: result.locales_count,
        message: if total_work_files == 0 {
            "Sync complete, no changes".to_string()
        } else {
            "Sync complete".to_string()
        },
        done: true,
        error: None,
    });

    Ok(result)
}

#[cfg(feature = "server")]
#[derive(Debug, Clone)]
struct SyncFile {
    path: String,
    sha: String,
    locale: Option<String>,
}

#[cfg(feature = "server")]
#[derive(Debug, Clone)]
struct ParsedFluentEntry {
    key: String,
    file_path: String,
    value: String,
}

#[cfg(feature = "server")]
#[derive(Debug, Clone)]
struct SyncTranslationValue {
    key_id: String,
    locale: String,
    file_path: String,
    value: String,
}

#[cfg(feature = "server")]
async fn fetch_existing_file_shas(
    db: &crate::auth::Db,
    project_id: &str,
) -> ServerFnResult<BTreeMap<String, String>> {
    let files = db
        .client
        .project_files
        .find_many(|project_file| project_file.where_project_id(project_id.to_string()))
        .await
        .map_err(|error| ServerFnError::new(error.to_string()))?;

    Ok(files
        .into_iter()
        .map(|file| (file.path, file.sha))
        .collect())
}

#[cfg(feature = "server")]
async fn upsert_project_files_many(
    db: &crate::auth::Db,
    project_id: &str,
    files: &[SyncFile],
    now: chrono::DateTime<chrono::Utc>,
) -> ServerFnResult<()> {
    if files.is_empty() {
        return Ok(());
    }

    db.client
        .project_files
        .upsert_many(
            files.to_vec(),
            |row| (row.project_id.clone(), row.path.clone()),
            |project_file, file| {
                project_file
                    .set_id(uuid::Uuid::new_v4().to_string())
                    .set_project_id(project_id.to_string())
                    .set_path(file.path.clone())
                    .set_locale(file.locale.clone())
                    .set_sha(file.sha.clone())
                    .set_created_at(now)
                    .set_updated_at(now)
            },
            |project_file, file| {
                project_file
                    .set_locale(file.locale.clone())
                    .set_sha(file.sha.clone())
                    .set_updated_at(now)
            },
        )
        .await
        .map_err(|error| ServerFnError::new(error.to_string()))?;

    Ok(())
}

#[cfg(feature = "server")]
async fn upsert_translation_keys_many(
    db: &crate::auth::Db,
    project_id: &str,
    entries: &[ParsedFluentEntry],
    now: chrono::DateTime<chrono::Utc>,
) -> ServerFnResult<()> {
    if entries.is_empty() {
        return Ok(());
    }

    db.client
        .translation_keys
        .upsert_many(
            entries.to_vec(),
            |row| (row.project_id.clone(), row.key.clone()),
            |translation_key, entry| {
                translation_key
                    .set_id(uuid::Uuid::new_v4().to_string())
                    .set_project_id(project_id.to_string())
                    .set_key(entry.key.clone())
                    .set_source_file_path(entry.file_path.clone())
                    .set_source_value(entry.value.clone())
                    .set_created_at(now)
                    .set_updated_at(now)
            },
            |translation_key, entry| {
                translation_key
                    .set_source_file_path(entry.file_path.clone())
                    .set_source_value(entry.value.clone())
                    .set_updated_at(now)
            },
        )
        .await
        .map_err(|error| ServerFnError::new(error.to_string()))?;

    Ok(())
}

#[cfg(feature = "server")]
async fn fetch_translation_key_ids(
    db: &crate::auth::Db,
    project_id: &str,
) -> ServerFnResult<BTreeMap<String, String>> {
    let keys = db
        .client
        .translation_keys
        .find_many(|translation_key| translation_key.where_project_id(project_id.to_string()))
        .await
        .map_err(|error| ServerFnError::new(error.to_string()))?;

    Ok(keys
        .into_iter()
        .map(|translation_key| (translation_key.key, translation_key.id))
        .collect())
}

#[cfg(feature = "server")]
async fn upsert_translation_values_many(
    db: &crate::auth::Db,
    project_id: &str,
    values: &[SyncTranslationValue],
    now: chrono::DateTime<chrono::Utc>,
) -> ServerFnResult<()> {
    if values.is_empty() {
        return Ok(());
    }

    db.client
        .translation_values
        .upsert_many(
            values.to_vec(),
            |row| {
                (
                    row.project_id.clone(),
                    row.key_id.clone(),
                    row.locale.clone(),
                )
            },
            |translation_value, value| {
                translation_value
                    .set_id(uuid::Uuid::new_v4().to_string())
                    .set_project_id(project_id.to_string())
                    .set_key_id(value.key_id.clone())
                    .set_locale(value.locale.clone())
                    .set_file_path(value.file_path.clone())
                    .set_value(value.value.clone())
                    .set_created_at(now)
                    .set_updated_at(now)
            },
            |translation_value, value| {
                translation_value
                    .set_file_path(value.file_path.clone())
                    .set_value(value.value.clone())
                    .set_updated_at(now)
            },
        )
        .await
        .map_err(|error| ServerFnError::new(error.to_string()))?;

    Ok(())
}

#[cfg(feature = "server")]
fn dedupe_entries_by_key(entries: &[ParsedFluentEntry]) -> Vec<ParsedFluentEntry> {
    let mut by_key = BTreeMap::<String, ParsedFluentEntry>::new();

    for entry in entries {
        by_key.insert(entry.key.clone(), entry.clone());
    }

    by_key.into_values().collect()
}

#[cfg(feature = "server")]
fn parse_fluent_entries(file_path: &str, content: &str) -> Vec<ParsedFluentEntry> {
    let mut entries = Vec::new();
    let mut current_key: Option<String> = None;
    let mut current_value = String::new();

    for line in content.lines() {
        let trimmed = line.trim();

        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }

        if !line.starts_with(' ') && !line.starts_with('\t') {
            if let Some(key) = current_key.take() {
                entries.push(ParsedFluentEntry {
                    key,
                    file_path: file_path.to_string(),
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
        entries.push(ParsedFluentEntry {
            key,
            file_path: file_path.to_string(),
            value: current_value.trim().to_string(),
        });
    }

    entries
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
            (language.len() == 2 || language.len() == 3) && (region.len() == 2 || region.len() == 4)
        }
        _ => false,
    }
}

#[cfg(feature = "server")]
async fn cleanup_deleted_files(
    db: &crate::auth::Db,
    project_id: &str,
    source_locale: &str,
    deleted_file_paths: &[String],
) -> ServerFnResult<()> {
    for path in deleted_file_paths {
        delete_translation_values_for_file(db, project_id, path).await?;

        let file_locale = infer_locale_from_project_path(path, Some(source_locale));

        if file_locale.as_deref() == Some(source_locale) {
            delete_translation_keys_for_source_file(db, project_id, path).await?;
        }

        delete_project_file(db, project_id, path).await?;
    }

    Ok(())
}

#[cfg(feature = "server")]
async fn delete_translation_values_for_file(
    db: &crate::auth::Db,
    project_id: &str,
    file_path: &str,
) -> ServerFnResult<()> {
    db.client
        .translation_values
        .delete(|translation_value| {
            translation_value
                .where_project_id(project_id.to_string())
                .where_file_path(file_path.to_string())
        })
        .await
        .map_err(|error| ServerFnError::new(error.to_string()))?;

    Ok(())
}

#[cfg(feature = "server")]
async fn delete_translation_keys_for_source_file(
    db: &crate::auth::Db,
    project_id: &str,
    source_file_path: &str,
) -> ServerFnResult<()> {
    db.client
        .translation_keys
        .delete(|translation_key| {
            translation_key
                .where_project_id(project_id.to_string())
                .where_source_file_path(source_file_path.to_string())
        })
        .await
        .map_err(|error| ServerFnError::new(error.to_string()))?;

    Ok(())
}

#[cfg(feature = "server")]
async fn delete_project_file(
    db: &crate::auth::Db,
    project_id: &str,
    path: &str,
) -> ServerFnResult<()> {
    db.client
        .project_files
        .delete(|project_file| {
            project_file
                .where_project_id(project_id.to_string())
                .where_path(path.to_string())
        })
        .await
        .map_err(|error| ServerFnError::new(error.to_string()))?;

    Ok(())
}

#[cfg(feature = "server")]
fn source_keys_by_file(entries: &[ParsedFluentEntry]) -> BTreeMap<String, BTreeSet<String>> {
    let mut by_file = BTreeMap::<String, BTreeSet<String>>::new();

    for entry in entries {
        by_file
            .entry(entry.file_path.clone())
            .or_default()
            .insert(entry.key.clone());
    }

    by_file
}

#[cfg(feature = "server")]
async fn cleanup_missing_source_keys_in_changed_files(
    db: &crate::auth::Db,
    project_id: &str,
    changed_source_file_paths: &[String],
    current_source_keys_by_file: &BTreeMap<String, BTreeSet<String>>,
) -> ServerFnResult<()> {
    for file_path in changed_source_file_paths {
        let current_keys = current_source_keys_by_file
            .get(file_path)
            .cloned()
            .unwrap_or_default();

        let existing_keys = db
            .client
            .translation_keys
            .find_many(|translation_key| {
                translation_key
                    .where_project_id(project_id.to_string())
                    .where_source_file_path(file_path.clone())
            })
            .await
            .map_err(|error| ServerFnError::new(error.to_string()))?;

        for existing_key in existing_keys {
            if !current_keys.contains(&existing_key.key) {
                delete_translation_key_by_id(db, &existing_key.id).await?;
            }
        }
    }

    Ok(())
}

#[cfg(feature = "server")]
async fn delete_translation_key_by_id(db: &crate::auth::Db, key_id: &str) -> ServerFnResult<()> {
    db.client
        .translation_keys
        .delete(|translation_key| translation_key.where_id(key_id.to_string()))
        .await
        .map_err(|error| ServerFnError::new(error.to_string()))?;

    Ok(())
}
