use super::dto::ProjectDto;
use dioxus::prelude::dioxus_fullstack::HeaderMap;
use dioxus::prelude::*;

#[get("/api/projects/:project_id", headers: HeaderMap)]
pub async fn fetch_project(project_id: String) -> ServerFnResult<Option<ProjectDto>> {
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
            .find_first(|project| project.where_id(project_id).where_owner_id(session.user_id))
            .await
            .map_err(|error| ServerFnError::new(error.to_string()))?;

        Ok(project.map(map_project))
    }

    #[cfg(not(feature = "server"))]
    {
        Err(ServerFnError::new(
            "fetch_project can only run on the server",
        ))
    }
}

#[get("/api/projects", headers: HeaderMap)]
pub async fn fetch_projects() -> ServerFnResult<Vec<ProjectDto>> {
    #[cfg(feature = "server")]
    {
        use super::dto::map_project;
        use super::helpers::session_cookie;

        dotenvy::dotenv().ok();

        let Some(session_token) = session_cookie(&headers, "bitefluent.session") else {
            return Ok(Vec::new());
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
            return Ok(Vec::new());
        };

        let projects = db
            .client
            .projects
            .find_many(|project| project.where_owner_id(session.user_id))
            .await
            .map_err(|error| ServerFnError::new(error.to_string()))?;

        Ok(projects.into_iter().map(map_project).collect())
    }

    #[cfg(not(feature = "server"))]
    {
        Err(ServerFnError::new(
            "fetch_projects can only run on the server",
        ))
    }
}
