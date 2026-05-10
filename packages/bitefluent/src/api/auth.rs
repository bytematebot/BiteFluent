use dioxus::prelude::dioxus_fullstack::HeaderMap;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuthUserDto {
    pub id: String,
    pub name: Option<String>,
    pub email: Option<String>,
    pub image: Option<String>,
}

#[get("/api/auth/session", headers: HeaderMap)]
pub async fn fetch_current_user() -> ServerFnResult<Option<AuthUserDto>> {
    #[cfg(feature = "server")]
    {
        use crate::auth::{BiteFluentAuthAdapter, BiteFluentSessionStore, Db};
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

        let adapter = BiteFluentAuthAdapter::new(db);
        let Some(user) = adapter
            .get_user_by_id(&session.user_id)
            .await
            .map_err(|error| ServerFnError::new(error.to_string()))?
        else {
            return Ok(None);
        };

        return Ok(Some(AuthUserDto {
            id: user.id,
            name: user.name,
            email: user.email,
            image: user.image,
        }));
    }

    #[cfg(not(feature = "server"))]
    {
        Err(ServerFnError::new(
            "fetch_current_user can only run on the server",
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
