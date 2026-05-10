use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use time::OffsetDateTime;

use crate::adapter::AuthAdapter;
use crate::error::{AuthError, AuthResult};
use crate::models::{AuthAccount, AuthUser, NewAuthAccount, NewAuthUser};

/// In-memory auth adapter.
///
/// Useful for tests, demos, and local development.
/// Do not use in production: all users and accounts are lost on restart.
#[derive(Debug, Clone, Default)]
pub struct MemoryAuthAdapter {
    inner: Arc<RwLock<MemoryAuthAdapterState>>,
}

#[derive(Debug, Default)]
struct MemoryAuthAdapterState {
    users: HashMap<String, AuthUser>,
    accounts: HashMap<String, AuthAccount>,
    next_user_id: u64,
    next_account_id: u64,
}

impl MemoryAuthAdapter {
    pub fn new() -> Self {
        Self::default()
    }

    fn account_key(provider: &str, provider_account_id: &str) -> String {
        format!("{provider}:{provider_account_id}")
    }

    fn next_user_id(state: &mut MemoryAuthAdapterState) -> String {
        state.next_user_id += 1;
        state.next_user_id.to_string()
    }

    fn next_account_id(state: &mut MemoryAuthAdapterState) -> String {
        state.next_account_id += 1;
        state.next_account_id.to_string()
    }
}

#[async_trait::async_trait]
impl AuthAdapter for MemoryAuthAdapter {
    async fn get_user_by_id(&self, user_id: &str) -> AuthResult<Option<AuthUser>> {
        let state = self
            .inner
            .read()
            .map_err(|_| AuthError::Adapter("memory adapter lock poisoned".to_string()))?;

        Ok(state.users.get(user_id).cloned())
    }

    async fn get_user_by_email(&self, email: &str) -> AuthResult<Option<AuthUser>> {
        let state = self
            .inner
            .read()
            .map_err(|_| AuthError::Adapter("memory adapter lock poisoned".to_string()))?;

        Ok(state
            .users
            .values()
            .find(|user| user.email.as_deref() == Some(email))
            .cloned())
    }

    async fn get_user_by_account(
        &self,
        provider: &str,
        provider_account_id: &str,
    ) -> AuthResult<Option<AuthUser>> {
        let state = self
            .inner
            .read()
            .map_err(|_| AuthError::Adapter("memory adapter lock poisoned".to_string()))?;

        let account_key = Self::account_key(provider, provider_account_id);

        let Some(account) = state.accounts.get(&account_key) else {
            return Ok(None);
        };

        Ok(state.users.get(&account.user_id).cloned())
    }

    async fn create_user(&self, user: NewAuthUser) -> AuthResult<AuthUser> {
        let mut state = self
            .inner
            .write()
            .map_err(|_| AuthError::Adapter("memory adapter lock poisoned".to_string()))?;

        let now = OffsetDateTime::now_utc();

        let auth_user = AuthUser {
            id: Self::next_user_id(&mut state),
            name: user.name,
            email: user.email,
            email_verified: user.email_verified,
            image: user.image,
            created_at: now,
            updated_at: now,
        };

        state.users.insert(auth_user.id.clone(), auth_user.clone());

        Ok(auth_user)
    }

    async fn link_account(&self, account: NewAuthAccount) -> AuthResult<AuthAccount> {
        let mut state = self
            .inner
            .write()
            .map_err(|_| AuthError::Adapter("memory adapter lock poisoned".to_string()))?;

        if !state.users.contains_key(&account.user_id) {
            return Err(AuthError::UserNotFound);
        }

        let now = OffsetDateTime::now_utc();

        let auth_account = AuthAccount {
            id: Self::next_account_id(&mut state),
            user_id: account.user_id,
            provider: account.provider,
            provider_account_id: account.provider_account_id,
            access_token: account.access_token,
            refresh_token: account.refresh_token,
            expires_at: account.expires_at,
            token_type: account.token_type,
            scope: account.scope,
            id_token: account.id_token,
            created_at: now,
            updated_at: now,
        };

        let account_key =
            Self::account_key(&auth_account.provider, &auth_account.provider_account_id);

        state.accounts.insert(account_key, auth_account.clone());

        Ok(auth_account)
    }

    async fn get_account(
        &self,
        provider: &str,
        provider_account_id: &str,
    ) -> AuthResult<Option<AuthAccount>> {
        let state = self
            .inner
            .read()
            .map_err(|_| AuthError::Adapter("memory adapter lock poisoned".to_string()))?;

        let account_key = Self::account_key(provider, provider_account_id);

        Ok(state.accounts.get(&account_key).cloned())
    }
}
