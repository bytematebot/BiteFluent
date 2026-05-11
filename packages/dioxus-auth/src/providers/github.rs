use serde::Deserialize;
use url::Url;

use crate::error::{AuthError, AuthResult};
use crate::models::{OAuthProfile, TokenSet};
use crate::provider::OAuthProvider;

#[derive(Debug, Clone)]
pub struct GitHubProvider {
    client_id: String,
    client_secret: String,
    scopes: Vec<&'static str>,
    http: reqwest::Client,
}

impl GitHubProvider {
    pub fn new(client_id: impl Into<String>, client_secret: impl Into<String>) -> Self {
        Self {
            client_id: client_id.into(),
            client_secret: client_secret.into(),
            scopes: vec!["read:user", "user:email", "repo", "read:org"],
            http: reqwest::Client::new(),
        }
    }

    pub fn scopes(mut self, scopes: Vec<&'static str>) -> Self {
        self.scopes = scopes;
        self
    }
}

#[async_trait::async_trait]
impl OAuthProvider for GitHubProvider {
    fn id(&self) -> &'static str {
        "github"
    }

    fn name(&self) -> &'static str {
        "GitHub"
    }

    fn scopes(&self) -> &[&'static str] {
        &self.scopes
    }

    fn authorization_url(&self, state: &str, redirect_uri: &str) -> AuthResult<String> {
        let mut url = Url::parse("https://github.com/login/oauth/authorize")?;

        url.query_pairs_mut()
            .append_pair("client_id", &self.client_id)
            .append_pair("redirect_uri", redirect_uri)
            .append_pair("scope", &self.scopes.join(" "))
            .append_pair("state", state);

        Ok(url.to_string())
    }

    async fn exchange_code(&self, code: &str, redirect_uri: &str) -> AuthResult<TokenSet> {
        let response = self
            .http
            .post("https://github.com/login/oauth/access_token")
            .header("Accept", "application/json")
            .form(&GitHubTokenRequest {
                client_id: &self.client_id,
                client_secret: &self.client_secret,
                code,
                redirect_uri,
            })
            .send()
            .await?
            .error_for_status()
            .map_err(|error| AuthError::TokenExchange(error.to_string()))?;

        let token_response = response.json::<GitHubTokenResponse>().await?;

        if let Some(error) = token_response.error {
            return Err(AuthError::TokenExchange(
                token_response.error_description.unwrap_or(error),
            ));
        }

        let access_token = token_response
            .access_token
            .ok_or_else(|| AuthError::TokenExchange("missing access_token".to_string()))?;

        Ok(TokenSet {
            access_token,
            refresh_token: None,
            expires_at: None,
            token_type: token_response.token_type,
            scope: token_response.scope,
            id_token: None,
        })
    }

    async fn profile(&self, tokens: &TokenSet) -> AuthResult<OAuthProfile> {
        let user = self
            .http
            .get("https://api.github.com/user")
            .bearer_auth(&tokens.access_token)
            .header("Accept", "application/vnd.github+json")
            .header("User-Agent", "dioxus-auth")
            .send()
            .await?
            .error_for_status()
            .map_err(|error| AuthError::ProfileFetch(error.to_string()))?
            .json::<GitHubUser>()
            .await?;

        let email = if user.email.is_some() {
            user.email.clone()
        } else {
            self.fetch_primary_email(&tokens.access_token).await?
        };

        Ok(OAuthProfile {
            provider: self.id().to_string(),
            provider_account_id: user.id.to_string(),
            name: user.name,
            username: Some(user.login),
            email,
            email_verified: None,
            image: user.avatar_url,
            raw: user.raw,
        })
    }
}

impl GitHubProvider {
    async fn fetch_primary_email(&self, access_token: &str) -> AuthResult<Option<String>> {
        let emails = self
            .http
            .get("https://api.github.com/user/emails")
            .bearer_auth(access_token)
            .header("Accept", "application/vnd.github+json")
            .header("User-Agent", "dioxus-auth")
            .send()
            .await?
            .error_for_status()
            .map_err(|error| AuthError::ProfileFetch(error.to_string()))?
            .json::<Vec<GitHubEmail>>()
            .await?;

        Ok(emails
            .into_iter()
            .find(|email| email.primary && email.verified)
            .map(|email| email.email))
    }
}

#[derive(serde::Serialize)]
struct GitHubTokenRequest<'a> {
    client_id: &'a str,
    client_secret: &'a str,
    code: &'a str,
    redirect_uri: &'a str,
}

#[derive(Debug, Deserialize)]
struct GitHubTokenResponse {
    access_token: Option<String>,
    token_type: Option<String>,
    scope: Option<String>,

    error: Option<String>,
    error_description: Option<String>,
}

#[derive(Debug, Deserialize)]
struct GitHubUser {
    id: u64,
    login: String,
    name: Option<String>,
    email: Option<String>,
    avatar_url: Option<String>,

    #[serde(flatten)]
    raw: serde_json::Value,
}

#[derive(Debug, Deserialize)]
struct GitHubEmail {
    email: String,
    primary: bool,
    verified: bool,
}
