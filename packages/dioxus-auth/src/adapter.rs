use async_trait::async_trait;

use crate::error::AuthResult;
use crate::models::{AuthAccount, AuthUser, NewAuthAccount, NewAuthUser};

#[async_trait]
pub trait AuthAdapter: Send + Sync {
    async fn get_user_by_id(&self, user_id: &str) -> AuthResult<Option<AuthUser>>;

    async fn get_user_by_email(&self, email: &str) -> AuthResult<Option<AuthUser>>;

    async fn get_user_by_account(
        &self,
        provider: &str,
        provider_account_id: &str,
    ) -> AuthResult<Option<AuthUser>>;

    async fn create_user(&self, user: NewAuthUser) -> AuthResult<AuthUser>;

    async fn link_account(&self, account: NewAuthAccount) -> AuthResult<AuthAccount>;

    async fn get_account(
        &self,
        provider: &str,
        provider_account_id: &str,
    ) -> AuthResult<Option<AuthAccount>>;
}
