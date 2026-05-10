use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use time::OffsetDateTime;

use crate::crypto::{generate_token, hash_token};
use crate::error::{AuthError, AuthResult};
use crate::models::{AuthSession, CreatedSession};
use crate::session::SessionStore;

/// In-memory session store.
///
/// Useful for tests, demos, and local development.
/// Do not use in production: all sessions are lost on restart.
#[derive(Debug, Clone, Default)]
pub struct MemorySessionStore {
    inner: Arc<RwLock<MemorySessionStoreState>>,
}

#[derive(Debug, Default)]
struct MemorySessionStoreState {
    sessions: HashMap<String, AuthSession>,
}

impl MemorySessionStore {
    pub fn new() -> Self {
        Self::default()
    }
}

#[async_trait::async_trait]
impl SessionStore for MemorySessionStore {
    async fn create_session(
        &self,
        user_id: &str,
        expires_at: OffsetDateTime,
    ) -> AuthResult<CreatedSession> {
        let token = generate_token();
        let token_hash = hash_token(&token);
        let now = OffsetDateTime::now_utc();

        let session = AuthSession {
            user_id: user_id.to_string(),
            session_token_hash: token_hash.clone(),
            expires_at,
            created_at: now,
            updated_at: now,
        };

        let mut state = self.inner.write().map_err(|_| {
            AuthError::SessionStore("memory session store lock poisoned".to_string())
        })?;

        state.sessions.insert(token_hash, session);

        Ok(CreatedSession { token, expires_at })
    }

    async fn get_session(&self, token: &str) -> AuthResult<Option<AuthSession>> {
        let token_hash = hash_token(token);

        let mut state = self.inner.write().map_err(|_| {
            AuthError::SessionStore("memory session store lock poisoned".to_string())
        })?;

        let Some(session) = state.sessions.get(&token_hash).cloned() else {
            return Ok(None);
        };

        if session.expires_at <= OffsetDateTime::now_utc() {
            state.sessions.remove(&token_hash);
            return Ok(None);
        }

        Ok(Some(session))
    }

    async fn delete_session(&self, token: &str) -> AuthResult<()> {
        let token_hash = hash_token(token);

        let mut state = self.inner.write().map_err(|_| {
            AuthError::SessionStore("memory session store lock poisoned".to_string())
        })?;

        state.sessions.remove(&token_hash);

        Ok(())
    }
}
