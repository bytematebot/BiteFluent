use super::dto::ProjectDto;
use dioxus::prelude::dioxus_fullstack::HeaderMap;
use dioxus::prelude::*;

#[post("/api/projects/import-github-repository", headers: HeaderMap)]
pub async fn import_github_repository(repository_id: u64) -> ServerFnResult<ProjectDto> {
    #[cfg(feature = "server")]
    {
        use super::dto::map_project;
        use super::helpers::{session_cookie, slugify};
        use crate::auth::{BiteFluentAuthAdapter, BiteFluentSessionStore, Db};
        use crate::integrations::github::GithubClient;
        use dioxus_auth::{AuthAdapter, SessionStore};
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

        let repositories = GithubClient::new(access_token)
            .repositories()
            .await
            .map_err(|error| ServerFnError::new(error.to_string()))?;

        let Some(repository) = repositories
            .into_iter()
            .find(|repository| repository.id == repository_id)
        else {
            return Err(ServerFnError::new("repository not found"));
        };

        let now = OffsetDateTime::now_utc();
        let now = crate::auth::time_to_chrono(now)
            .map_err(|error| ServerFnError::new(error.to_string()))?;

        let slug = slugify(&repository.name);

        if let Some(existing) = db
            .client
            .projects
            .find_first(|project| {
                project
                    .where_owner_id(session.user_id.clone())
                    .where_provider("github".to_string())
                    .where_repository_id(repository.id as i64)
            })
            .await
            .map_err(|error| ServerFnError::new(error.to_string()))?
        {
            return Ok(map_project(existing));
        }

        let created = db
            .client
            .projects
            .create(|project| {
                project
                    .set_id(uuid::Uuid::new_v4().to_string())
                    .set_owner_id(session.user_id.clone())
                    .set_name(repository.name.clone())
                    .set_slug(slug)
                    .set_provider("github".to_string())
                    .set_repository_id(repository.id as i64)
                    .set_repository_owner(repository.owner.clone())
                    .set_repository_name(repository.name.clone())
                    .set_repository_full_name(repository.full_name.clone())
                    .set_repository_private(repository.private)
                    .set_default_branch(repository.default_branch.clone())
                    .set_source_locale(None)
                    .set_locales_path(None)
                    .set_created_at(now)
                    .set_updated_at(now)
            })
            .await
            .map_err(|error| ServerFnError::new(error.to_string()))?;

        Ok(map_project(created))
    }

    #[cfg(not(feature = "server"))]
    {
        Err(ServerFnError::new(
            "import_github_repository can only run on the server",
        ))
    }
}

#[get("/api/projects/onboarding/current", headers: HeaderMap)]
pub async fn fetch_current_onboarding_project() -> ServerFnResult<Option<ProjectDto>> {
    #[cfg(feature = "server")]
    {
        use super::dto::map_project;
        use super::helpers::session_cookie;

        dotenvy::dotenv().ok();

        let Some(session_token) = session_cookie(&headers, "bitefluent.session") else {
            return Ok(None);
        };

        let database_url =
            std::env::var("DATABASE_URL").map_err(|error| ServerFnError::new(error.to_string()))?;

        let client = byteorm_client::Client::new(&database_url)
            .await
            .map_err(|error| ServerFnError::new(error.to_string()))?;

        let db = crate::auth::Db::new(client);
        let session_store = crate::auth::BiteFluentSessionStore::new(db.clone());

        let Some(session) = dioxus_auth::SessionStore::get_session(&session_store, &session_token)
            .await
            .map_err(|error| ServerFnError::new(error.to_string()))?
        else {
            return Ok(None);
        };

        let project = db
            .client
            .projects
            .find_first(|project| {
                project
                    .where_owner_id(session.user_id)
                    .where_source_locale(None)
            })
            .await
            .map_err(|error| ServerFnError::new(error.to_string()))?;

        Ok(project.map(map_project))
    }

    #[cfg(not(feature = "server"))]
    {
        Err(ServerFnError::new(
            "fetch_current_onboarding_project can only run on the server",
        ))
    }
}

#[post("/api/projects/finish-onboarding", headers: HeaderMap)]
pub async fn finish_onboarding(
    project_id: String,
    locales_path: String,
    source_locale: String,
) -> ServerFnResult<ProjectDto> {
    #[cfg(feature = "server")]
    {
        use super::dto::map_project;
        use super::helpers::session_cookie;
        use crate::auth::{BiteFluentSessionStore, Db};
        use dioxus_auth::SessionStore;
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

        let now = OffsetDateTime::now_utc();
        let now = crate::auth::time_to_chrono(now)
            .map_err(|error| ServerFnError::new(error.to_string()))?;

        let project = db
            .client
            .projects
            .update(|project| {
                project
                    .where_id(project_id)
                    .where_owner_id(session.user_id.clone())
                    .set_locales_path(Some(locales_path))
                    .set_source_locale(Some(source_locale))
                    .set_updated_at(now)
            })
            .await
            .map_err(|error| ServerFnError::new(error.to_string()))?;

        db.client
            .users
            .update(|user| {
                user.where_id(session.user_id)
                    .set_onboarded(true)
                    .set_updated_at(now)
            })
            .await
            .map_err(|error| ServerFnError::new(error.to_string()))?;

        Ok(map_project(project))
    }

    #[cfg(not(feature = "server"))]
    {
        Err(ServerFnError::new(
            "finish_onboarding can only run on the server",
        ))
    }
}
