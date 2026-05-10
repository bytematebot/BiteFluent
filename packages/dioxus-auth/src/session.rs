use async_trait::async_trait;
use time::OffsetDateTime;

use crate::error::{AuthError, AuthResult};
use crate::models::{AuthSession, CreatedSession};

#[async_trait]
pub trait SessionStore: Send + Sync {
    async fn create_session(
        &self,
        user_id: &str,
        expires_at: OffsetDateTime,
    ) -> AuthResult<CreatedSession>;

    async fn get_session(&self, token: &str) -> AuthResult<Option<AuthSession>>;

    async fn delete_session(&self, token: &str) -> AuthResult<()>;

    async fn rotate_session(
        &self,
        old_token: &str,
        expires_at: OffsetDateTime,
    ) -> AuthResult<CreatedSession> {
        let session = self
            .get_session(old_token)
            .await?
            .ok_or(AuthError::InvalidSession)?;

        self.delete_session(old_token).await?;
        self.create_session(&session.user_id, expires_at).await
    }
}
