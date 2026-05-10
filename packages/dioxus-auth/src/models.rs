use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

/// A normalized user stored by the auth adapter.
///
/// This is intentionally provider-agnostic. A single user can have many linked
/// OAuth accounts, for example GitHub + Discord + Google.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthUser {
    pub id: String,
    pub name: Option<String>,
    pub email: Option<String>,
    pub email_verified: Option<OffsetDateTime>,
    pub image: Option<String>,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

/// Data required to create a new user.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NewAuthUser {
    pub name: Option<String>,
    pub email: Option<String>,
    pub email_verified: Option<OffsetDateTime>,
    pub image: Option<String>,
}

/// A linked OAuth account.
///
/// Example:
/// - provider: "github"
/// - provider_account_id: GitHub user ID
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthAccount {
    pub id: String,
    pub user_id: String,

    pub provider: String,
    pub provider_account_id: String,

    pub access_token: Option<String>,
    pub refresh_token: Option<String>,
    pub expires_at: Option<OffsetDateTime>,
    pub token_type: Option<String>,
    pub scope: Option<String>,
    pub id_token: Option<String>,

    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

/// Data required to link a new OAuth account to a user.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NewAuthAccount {
    pub user_id: String,

    pub provider: String,
    pub provider_account_id: String,

    pub access_token: Option<String>,
    pub refresh_token: Option<String>,
    pub expires_at: Option<OffsetDateTime>,
    pub token_type: Option<String>,
    pub scope: Option<String>,
    pub id_token: Option<String>,
}

/// A server-side auth session.
///
/// The raw session token should never be stored directly.
/// Store only a hash of the token.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthSession {
    pub user_id: String,
    pub session_token_hash: String,
    pub expires_at: OffsetDateTime,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

/// The returned session after creating one.
///
/// `token` is the raw token that should be sent to the browser as an HttpOnly
/// cookie. The session store should only persist its hash.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CreatedSession {
    pub token: String,
    pub expires_at: OffsetDateTime,
}

/// OAuth token response normalized across providers.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TokenSet {
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub expires_at: Option<OffsetDateTime>,
    pub token_type: Option<String>,
    pub scope: Option<String>,
    pub id_token: Option<String>,
}

/// Normalized profile returned by an OAuth provider.
/// Providers should map their provider-specific response into this shape.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OAuthProfile {
    pub provider: String,
    pub provider_account_id: String,

    pub name: Option<String>,
    pub username: Option<String>,
    pub email: Option<String>,
    pub email_verified: Option<bool>,
    pub image: Option<String>,

    pub raw: serde_json::Value,
}

impl OAuthProfile {
    pub fn to_new_user(&self) -> NewAuthUser {
        NewAuthUser {
            name: self.name.clone().or_else(|| self.username.clone()),
            email: self.email.clone(),
            email_verified: self
                .email_verified
                .and_then(|verified| verified.then_some(OffsetDateTime::now_utc())),
            image: self.image.clone(),
        }
    }
}
