use std::collections::HashMap;
use std::sync::Arc;

use time::OffsetDateTime;

use crate::adapter::AuthAdapter;
use crate::config::AuthConfig;
use crate::crypto::generate_token;
use crate::error::{AuthError, AuthResult};
use crate::models::{
    AuthSession, AuthUser, CreatedSession, NewAuthAccount, OAuthProfile, TokenSet,
};
use crate::provider::OAuthProvider;
use crate::session::SessionStore;

#[derive(Clone)]
pub struct Auth<A, S> {
    config: AuthConfig,
    adapter: A,
    session_store: S,
    providers: HashMap<String, Arc<dyn OAuthProvider>>,
}

#[derive(Debug, Clone)]
pub struct AuthorizationUrl {
    pub provider: String,
    pub url: String,
    pub state: String,
}

#[derive(Debug, Clone)]
pub struct AuthCallbackResult {
    pub user: AuthUser,
    pub session: CreatedSession,
    pub is_new_user: bool,
    pub is_new_account: bool,
}

#[derive(Debug, Clone)]
pub struct CurrentSession {
    pub user: AuthUser,
    pub session: AuthSession,
}

impl<A, S> Auth<A, S>
where
    A: AuthAdapter,
    S: SessionStore,
{
    pub fn new(config: AuthConfig, adapter: A, session_store: S) -> Self {
        Self {
            config,
            adapter,
            session_store,
            providers: HashMap::new(),
        }
    }

    pub fn with_provider<P>(mut self, provider: P) -> Self
    where
        P: OAuthProvider + 'static,
    {
        self.providers
            .insert(provider.id().to_string(), Arc::new(provider));

        self
    }

    pub fn config(&self) -> &AuthConfig {
        &self.config
    }

    pub fn adapter(&self) -> &A {
        &self.adapter
    }

    pub fn session_store(&self) -> &S {
        &self.session_store
    }

    pub fn provider(&self, provider_id: &str) -> AuthResult<&dyn OAuthProvider> {
        self.providers
            .get(provider_id)
            .map(|provider| provider.as_ref())
            .ok_or_else(|| AuthError::ProviderNotFound(provider_id.to_string()))
    }

    pub fn create_authorization_url(&self, provider_id: &str) -> AuthResult<AuthorizationUrl> {
        let state = generate_token();
        let url = self.authorization_url_with_state(provider_id, &state)?;

        Ok(AuthorizationUrl {
            provider: provider_id.to_string(),
            url,
            state,
        })
    }

    pub fn authorization_url_with_state(
        &self,
        provider_id: &str,
        state: &str,
    ) -> AuthResult<String> {
        let provider = self.provider(provider_id)?;
        let redirect_uri = self.config.callback_url(provider_id);

        provider.authorization_url(state, &redirect_uri)
    }

    pub async fn handle_oauth_callback(
        &self,
        provider_id: &str,
        code: &str,
    ) -> AuthResult<AuthCallbackResult> {
        let provider = self.provider(provider_id)?;
        let redirect_uri = self.config.callback_url(provider_id);

        let tokens = provider.exchange_code(code, &redirect_uri).await?;
        let profile = provider.profile(&tokens).await?;

        self.sign_in_with_oauth_profile(profile, tokens).await
    }

    pub async fn current_session(&self, session_token: &str) -> AuthResult<Option<CurrentSession>> {
        let Some(session) = self.session_store.get_session(session_token).await? else {
            return Ok(None);
        };

        let Some(user) = self.adapter.get_user_by_id(&session.user_id).await? else {
            return Ok(None);
        };

        Ok(Some(CurrentSession { user, session }))
    }

    pub async fn sign_out(&self, session_token: &str) -> AuthResult<()> {
        self.session_store.delete_session(session_token).await
    }

    async fn sign_in_with_oauth_profile(
        &self,
        profile: OAuthProfile,
        tokens: TokenSet,
    ) -> AuthResult<AuthCallbackResult> {
        let existing_user = self
            .adapter
            .get_user_by_account(&profile.provider, &profile.provider_account_id)
            .await?;

        let (user, is_new_user, is_new_account) = match existing_user {
            Some(user) => (user, false, false),
            None => {
                let user = self.adapter.create_user(profile.to_new_user()).await?;

                let account = NewAuthAccount {
                    user_id: user.id.clone(),
                    provider: profile.provider.clone(),
                    provider_account_id: profile.provider_account_id.clone(),
                    access_token: Some(tokens.access_token),
                    refresh_token: tokens.refresh_token,
                    expires_at: tokens.expires_at,
                    token_type: tokens.token_type,
                    scope: tokens.scope,
                    id_token: tokens.id_token,
                };

                self.adapter.link_account(account).await?;

                (user, true, true)
            }
        };

        let expires_at = OffsetDateTime::now_utc() + self.config.cookie.session_max_age;

        let session = self
            .session_store
            .create_session(&user.id, expires_at)
            .await?;

        Ok(AuthCallbackResult {
            user,
            session,
            is_new_user,
            is_new_account,
        })
    }
}
