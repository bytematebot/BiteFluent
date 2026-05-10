use async_trait::async_trait;

use crate::error::AuthResult;
use crate::models::{OAuthProfile, TokenSet};

#[async_trait]
pub trait OAuthProvider: Send + Sync {
    fn id(&self) -> &'static str;

    fn name(&self) -> &'static str {
        self.id()
    }

    fn scopes(&self) -> &[&'static str];

    fn authorization_url(&self, state: &str, redirect_uri: &str) -> AuthResult<String>;

    async fn exchange_code(&self, code: &str, redirect_uri: &str) -> AuthResult<TokenSet>;

    async fn profile(&self, tokens: &TokenSet) -> AuthResult<OAuthProfile>;
}
