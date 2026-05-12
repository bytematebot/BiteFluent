use dioxus::prelude::dioxus_fullstack::HeaderMap;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuthUserDto {
    pub id: String,
    pub display_name: String,
    pub email: Option<String>,
    pub image: Option<String>,
    pub onboarded: bool,
}

pub type AuthUserResource = Resource<Option<AuthUserDto>>;

#[derive(Debug, Clone, PartialEq)]
pub enum AuthUserState {
    Loading,
    Guest,
    Authenticated(AuthUserDto),
}

impl AuthUserState {
    pub fn is_loading(&self) -> bool {
        matches!(self, Self::Loading)
    }

    pub fn is_guest(&self) -> bool {
        matches!(self, Self::Guest)
    }

    pub fn is_authenticated(&self) -> bool {
        matches!(self, Self::Authenticated(_))
    }

    pub fn user(&self) -> Option<&AuthUserDto> {
        match self {
            Self::Authenticated(user) => Some(user),
            _ => None,
        }
    }

    pub fn from_resource(value: Option<Option<AuthUserDto>>) -> Self {
        match value {
            None => Self::Loading,
            Some(None) => Self::Guest,
            Some(Some(user)) => Self::Authenticated(user),
        }
    }
}

pub fn read_auth_user(resource: &AuthUserResource) -> AuthUserState {
    AuthUserState::from_resource(resource.read().clone())
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

        let Some(user) = db
            .client
            .users
            .find_first(|user| user.where_id(session.user_id.clone()))
            .await
            .map_err(|error| ServerFnError::new(error.to_string()))?
        else {
            return Ok(None);
        };

        let display_name = user
            .name
            .as_ref()
            .or(user.email.as_ref())
            .cloned()
            .unwrap_or_else(|| "GitHub user".to_string());

        return Ok(Some(AuthUserDto {
            id: user.id,
            display_name,
            email: user.email,
            image: user.image,
            onboarded: user.onboarded,
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
