//! Authentication primitives for BiteFluent / Dioxus fullstack apps.
//!
//! This crate provides provider-agnostic OAuth authentication,
//! adapter-based user persistence, and session-store abstraction.

pub mod adapter;
pub mod auth;
pub mod config;
pub mod crypto;
pub mod error;
pub mod models;
pub mod provider;
pub mod session;

pub mod adapters;
pub mod providers;
pub mod sessions;

#[cfg(feature = "axum")]
pub mod axum;

pub use adapter::AuthAdapter;
pub use auth::{Auth, AuthCallbackResult, AuthorizationUrl, CurrentSession};
pub use config::{AuthConfig, CookieConfig, SameSiteMode};
pub use error::{AuthError, AuthResult};
pub use models::{
    AuthAccount, AuthSession, AuthUser, CreatedSession, NewAuthAccount, NewAuthUser, OAuthProfile,
    TokenSet,
};
pub use provider::OAuthProvider;
pub use session::SessionStore;

pub mod prelude {
    pub use crate::{
        Auth, AuthAccount, AuthAdapter, AuthCallbackResult, AuthConfig, AuthError, AuthResult,
        AuthSession, AuthUser, AuthorizationUrl, CookieConfig, CreatedSession, CurrentSession,
        NewAuthAccount, NewAuthUser, OAuthProfile, OAuthProvider, SameSiteMode, SessionStore,
        TokenSet,
    };

    pub use crate::adapters::*;
    pub use crate::providers::*;
    pub use crate::sessions::*;
}
