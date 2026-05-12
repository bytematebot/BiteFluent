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
        use crate::auth::{BiteFluentSessionStore, Db};
        use dioxus_auth::SessionStore;

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

        let project_dto = map_project(project);

        let mut files = db
            .client
            .project_files
            .find_many(|file| file.where_project_id(project_dto.id.clone()))
            .await
            .map_err(|error| ServerFnError::new(error.to_string()))?
            .into_iter()
            .map(|file| ProjectWorkspaceFileDto {
                name: file
                    .path
                    .rsplit('/')
                    .next()
                    .unwrap_or(&file.path)
                    .to_string(),
                path: file.path,
                sha: file.sha,
                locale: file.locale,
            })
            .collect::<Vec<_>>();

        files.sort_by(|left, right| {
            left.locale
                .cmp(&right.locale)
                .then_with(|| left.path.cmp(&right.path))
        });

        let translation_keys = db
            .client
            .translation_keys
            .find_many(|key| key.where_project_id(project_dto.id.clone()))
            .await
            .map_err(|error| ServerFnError::new(error.to_string()))?;

        let mut keys = translation_keys
            .into_iter()
            .map(|key| ProjectWorkspaceKeyDto {
                key: key.key,
                file_path: key.source_file_path,
                locale: project_dto.source_locale.clone(),
                value: key.source_value,
            })
            .collect::<Vec<_>>();

        keys.sort_by(|left, right| {
            left.file_path
                .cmp(&right.file_path)
                .then_with(|| left.key.cmp(&right.key))
        });

        Ok(Some(ProjectWorkspaceDto {
            project: project_dto,
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
