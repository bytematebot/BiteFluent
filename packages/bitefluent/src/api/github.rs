use dioxus::prelude::dioxus_fullstack::HeaderMap;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GithubRepositoryDto {
    pub id: u64,
    pub owner: String,
    pub name: String,
    pub full_name: String,
    pub private: bool,
    pub default_branch: String,
}

#[get("/api/github/repositories", headers: HeaderMap)]
pub async fn fetch_github_repositories() -> ServerFnResult<Vec<GithubRepositoryDto>> {
    #[cfg(feature = "server")]
    {
        use crate::auth::{BiteFluentAuthAdapter, BiteFluentSessionStore, Db};
        use crate::integrations::github::GithubClient;
        use dioxus_auth::{AuthAdapter, SessionStore};

        dotenvy::dotenv().ok();

        let Some(session_token) = session_cookie(&headers, "bitefluent.session") else {
            return Ok(Vec::new());
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
            return Ok(Vec::new());
        };

        let adapter = BiteFluentAuthAdapter::new(db);

        let Some(account) = adapter
            .get_github_account_for_user(&session.user_id)
            .await
            .map_err(|error| ServerFnError::new(error.to_string()))?
        else {
            return Ok(Vec::new());
        };

        let Some(access_token) = account.access_token else {
            return Ok(Vec::new());
        };

        let repositories = GithubClient::new(access_token)
            .repositories()
            .await
            .map_err(|error| ServerFnError::new(error.to_string()))?;

        return Ok(repositories
            .into_iter()
            .map(|repo| GithubRepositoryDto {
                id: repo.id,
                owner: repo.owner,
                name: repo.name,
                full_name: repo.full_name,
                private: repo.private,
                default_branch: repo.default_branch,
            })
            .collect());
    }

    #[cfg(not(feature = "server"))]
    {
        Err(ServerFnError::new(
            "fetch_github_repositories can only run on the server",
        ))
    }
}

#[cfg(feature = "server")]
fn session_cookie(headers: &HeaderMap, name: &str) -> Option<String> {
    let cookie_header = headers.get("cookie")?.to_str().ok()?;

    cookie_header
        .split(';')
        .filter_map(|cookie| cookie.trim().split_once('='))
        .find_map(|(cookie_name, value)| (cookie_name == name).then(|| value.to_string()))
}
