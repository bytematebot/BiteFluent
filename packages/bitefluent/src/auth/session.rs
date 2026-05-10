use super::adapter::Db;
use crate::auth::{chrono_to_time, time_to_chrono};
use dioxus_auth::{
    AuthError, AuthResult, AuthSession, CreatedSession, SessionStore,
    crypto::{generate_token, hash_token},
};
use time::OffsetDateTime;

#[derive(Clone)]
pub struct BiteFluentSessionStore {
    db: Db,
}

impl BiteFluentSessionStore {
    pub fn new(db: Db) -> Self {
        Self { db }
    }
}

#[async_trait::async_trait]
impl SessionStore for BiteFluentSessionStore {
    async fn create_session(
        &self,
        user_id: &str,
        expires_at: OffsetDateTime,
    ) -> AuthResult<CreatedSession> {
        let token = generate_token();
        let token_hash = hash_token(&token);
        let now = OffsetDateTime::now_utc();

        let expires_at_chrono = time_to_chrono(expires_at)?;
        let now_chrono = time_to_chrono(now)?;

        self.db
            .client
            .sessions
            .create(|s| {
                s.set_session_token_hash(token_hash)
                    .set_user_id(user_id.to_string())
                    .set_expires_at(expires_at_chrono)
                    .set_created_at(now_chrono)
                    .set_updated_at(now_chrono)
            })
            .await
            .map_err(|err| AuthError::SessionStore(err.to_string()))?;

        Ok(CreatedSession { token, expires_at })
    }

    async fn get_session(&self, token: &str) -> AuthResult<Option<AuthSession>> {
        let token_hash = hash_token(token);

        let session = self
            .db
            .client
            .sessions
            .find_first(|s| s.where_session_token_hash(token_hash.clone()))
            .await
            .map_err(|err| AuthError::SessionStore(err.to_string()))?;

        let Some(session) = session else {
            return Ok(None);
        };

        let session = map_session(session)?;

        if session.expires_at <= OffsetDateTime::now_utc() {
            self.delete_session(token).await?;
            return Ok(None);
        }

        Ok(Some(session))
    }

    async fn delete_session(&self, token: &str) -> AuthResult<()> {
        let token_hash = hash_token(token);

        self.db
            .client
            .sessions
            .delete(|s| s.where_session_token_hash(token_hash))
            .await
            .map_err(|err| AuthError::SessionStore(err.to_string()))?;

        Ok(())
    }
}

fn map_session(session: byteorm_client::Sessions) -> AuthResult<AuthSession> {
    Ok(AuthSession {
        user_id: session.user_id,
        session_token_hash: session.session_token_hash,
        expires_at: chrono_to_time(session.expires_at)?,
        created_at: chrono_to_time(session.created_at)?,
        updated_at: chrono_to_time(session.updated_at)?,
    })
}
