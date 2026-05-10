use thiserror::Error;

pub type AuthResult<T> = Result<T, AuthError>;

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("provider not found: {0}")]
    ProviderNotFound(String),

    #[error("invalid OAuth state")]
    InvalidState,

    #[error("missing OAuth code")]
    MissingCode,

    #[error("missing session cookie")]
    MissingSessionCookie,

    #[error("invalid session")]
    InvalidSession,

    #[error("session expired")]
    SessionExpired,

    #[error("user not found")]
    UserNotFound,

    #[error("account not found")]
    AccountNotFound,

    #[error("failed to create authorization URL: {0}")]
    AuthorizationUrl(String),

    #[error("failed to exchange OAuth code: {0}")]
    TokenExchange(String),

    #[error("failed to fetch OAuth profile: {0}")]
    ProfileFetch(String),

    #[error("request failed: {0}")]
    Request(#[from] reqwest::Error),

    #[error("url parse failed: {0}")]
    Url(#[from] url::ParseError),

    #[error("json serialization failed: {0}")]
    Json(#[from] serde_json::Error),

    #[error("adapter error: {0}")]
    Adapter(String),

    #[error("session store error: {0}")]
    SessionStore(String),

    #[error("crypto error: {0}")]
    Crypto(String),

    #[error("configuration error: {0}")]
    Config(String),

    #[error("internal auth error: {0}")]
    Internal(String),
}
