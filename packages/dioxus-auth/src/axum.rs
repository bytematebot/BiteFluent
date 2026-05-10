use std::sync::Arc;

use axum::{
    Json, Router,
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Redirect, Response},
    routing::get,
};
use serde::Deserialize;
use time::OffsetDateTime;
use tower_cookies::{Cookie, Cookies};

use crate::{Auth, AuthAdapter, AuthConfig, AuthError, SameSiteMode, SessionStore};

#[derive(Clone)]
pub struct AuthState<A, S> {
    auth: Arc<Auth<A, S>>,
}

impl<A, S> AuthState<A, S> {
    pub fn new(auth: Auth<A, S>) -> Self {
        Self {
            auth: Arc::new(auth),
        }
    }

    pub fn auth(&self) -> Arc<Auth<A, S>> {
        self.auth.clone()
    }
}

pub fn router<A, S>(auth: Auth<A, S>) -> Router
where
    A: AuthAdapter + Clone + Send + Sync + 'static,
    S: SessionStore + Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/signin/{provider}", get(sign_in::<A, S>))
        .route("/callback/{provider}", get(callback::<A, S>))
        .route("/signout", get(sign_out::<A, S>))
        .route("/session", get(session::<A, S>))
        .with_state(AuthState::new(auth))
}

async fn session<A, S>(
    State(state): State<AuthState<A, S>>,
    cookies: Cookies,
) -> Result<Response, AuthResponseError>
where
    A: AuthAdapter + Clone + Send + Sync + 'static,
    S: SessionStore + Clone + Send + Sync + 'static,
{
    let auth = state.auth();

    let Some(cookie) = cookies.get(&auth.config().cookie.session_cookie_name) else {
        return Ok(StatusCode::UNAUTHORIZED.into_response());
    };

    let Some(current_session) = auth.current_session(cookie.value()).await? else {
        cookies.remove(remove_cookie(
            auth.config(),
            auth.config().cookie.session_cookie_name.clone(),
        ));

        return Ok(StatusCode::UNAUTHORIZED.into_response());
    };

    Ok(Json(current_session.user).into_response())
}

async fn sign_in<A, S>(
    State(state): State<AuthState<A, S>>,
    Path(provider): Path<String>,
    cookies: Cookies,
) -> Result<Response, AuthResponseError>
where
    A: AuthAdapter + Clone + Send + Sync + 'static,
    S: SessionStore + Clone + Send + Sync + 'static,
{
    let auth = state.auth();

    let authorization = auth.create_authorization_url(&provider)?;

    let state_cookie = build_cookie(
        auth.config(),
        auth.config().cookie.state_cookie_name.clone(),
        authorization.state,
        OffsetDateTime::now_utc() + auth.config().cookie.state_max_age,
    );

    cookies.add(state_cookie);

    Ok(Redirect::temporary(&authorization.url).into_response())
}

async fn callback<A, S>(
    State(state): State<AuthState<A, S>>,
    Path(provider): Path<String>,
    Query(query): Query<AuthCallbackQuery>,
    cookies: Cookies,
) -> Result<Response, AuthResponseError>
where
    A: AuthAdapter + Clone + Send + Sync + 'static,
    S: SessionStore + Clone + Send + Sync + 'static,
{
    let auth = state.auth();

    let expected_state = cookies
        .get(&auth.config().cookie.state_cookie_name)
        .map(|cookie| cookie.value().to_string())
        .ok_or(AuthError::InvalidState)?;

    if expected_state != query.state {
        return Err(AuthError::InvalidState.into());
    }

    let result = auth.handle_oauth_callback(&provider, &query.code).await?;

    cookies.remove(remove_cookie(
        auth.config(),
        auth.config().cookie.state_cookie_name.clone(),
    ));

    let session_cookie = build_cookie(
        auth.config(),
        auth.config().cookie.session_cookie_name.clone(),
        result.session.token,
        result.session.expires_at,
    );

    cookies.add(session_cookie);

    Ok(Redirect::temporary("/start").into_response())
}

async fn sign_out<A, S>(
    State(state): State<AuthState<A, S>>,
    cookies: Cookies,
) -> Result<Response, AuthResponseError>
where
    A: AuthAdapter + Clone + Send + Sync + 'static,
    S: SessionStore + Clone + Send + Sync + 'static,
{
    let auth = state.auth();

    if let Some(cookie) = cookies.get(&auth.config().cookie.session_cookie_name) {
        auth.sign_out(cookie.value()).await?;
    }

    cookies.remove(remove_cookie(
        auth.config(),
        auth.config().cookie.session_cookie_name.clone(),
    ));

    Ok(Redirect::temporary("/").into_response())
}

#[derive(Debug, Deserialize)]
struct AuthCallbackQuery {
    code: String,
    state: String,
}

fn build_cookie(
    config: &AuthConfig,
    name: String,
    value: String,
    expires_at: OffsetDateTime,
) -> Cookie<'static> {
    let mut cookie = Cookie::new(name, value);

    cookie.set_path(config.cookie.path.clone());
    cookie.set_http_only(config.cookie.http_only);
    cookie.set_secure(config.cookie.secure);
    cookie.set_same_site(to_tower_same_site(config.cookie.same_site));
    cookie.set_expires(expires_at);

    cookie
}

fn remove_cookie(config: &AuthConfig, name: String) -> Cookie<'static> {
    let mut cookie = Cookie::new(name, "");

    cookie.set_path(config.cookie.path.clone());
    cookie.set_http_only(config.cookie.http_only);
    cookie.set_secure(config.cookie.secure);
    cookie.set_same_site(to_tower_same_site(config.cookie.same_site));
    cookie.set_expires(OffsetDateTime::UNIX_EPOCH);

    cookie
}

fn to_tower_same_site(mode: SameSiteMode) -> tower_cookies::cookie::SameSite {
    match mode {
        SameSiteMode::Lax => tower_cookies::cookie::SameSite::Lax,
        SameSiteMode::Strict => tower_cookies::cookie::SameSite::Strict,
        SameSiteMode::None => tower_cookies::cookie::SameSite::None,
    }
}

pub struct AuthResponseError(AuthError);

impl From<AuthError> for AuthResponseError {
    fn from(error: AuthError) -> Self {
        Self(error)
    }
}

impl IntoResponse for AuthResponseError {
    fn into_response(self) -> Response {
        let status = match self.0 {
            AuthError::InvalidState => StatusCode::BAD_REQUEST,
            AuthError::MissingCode => StatusCode::BAD_REQUEST,
            AuthError::ProviderNotFound(_) => StatusCode::NOT_FOUND,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };

        (status, self.0.to_string()).into_response()
    }
}
