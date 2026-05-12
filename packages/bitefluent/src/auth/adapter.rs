use crate::auth::datetime::{
    chrono_to_time, chrono_to_time_option, time_to_chrono, time_to_chrono_option,
};
use chrono::{DateTime, Utc};
use dioxus_auth::{
    AuthAccount, AuthAdapter, AuthError, AuthResult, AuthSession, AuthUser, NewAuthAccount,
    NewAuthUser,
};
use time::OffsetDateTime;
#[derive(Clone)]
pub struct BiteFluentAuthAdapter {
    db: Db,
}

#[derive(Clone)]
pub struct Db {
    pub(crate) client: byteorm_client::Client,
}

impl Db {
    pub fn new(client: byteorm_client::Client) -> Self {
        Self { client }
    }
}

impl BiteFluentAuthAdapter {
    pub fn new(db: Db) -> Self {
        Self { db }
    }
}

#[async_trait::async_trait]
impl AuthAdapter for BiteFluentAuthAdapter {
    async fn get_user_by_id(&self, user_id: &str) -> AuthResult<Option<AuthUser>> {
        let user = self
            .db
            .client
            .users
            .find_first(|u| u.where_id(user_id.to_string()))
            .await
            .map_err(|err| AuthError::Adapter(err.to_string()))?;

        user.map(map_user).transpose()
    }

    async fn get_user_by_email(&self, email: &str) -> AuthResult<Option<AuthUser>> {
        let user = self
            .db
            .client
            .users
            .find_first(|u| u.where_email(Some(email.to_string())))
            .await
            .map_err(|err| AuthError::Adapter(err.to_string()))?;

        user.map(map_user).transpose()
    }

    async fn get_user_by_account(
        &self,
        provider: &str,
        provider_account_id: &str,
    ) -> AuthResult<Option<AuthUser>> {
        let Some(account) = self.get_account(provider, provider_account_id).await? else {
            return Ok(None);
        };

        self.get_user_by_id(&account.user_id).await
    }

    async fn create_user(&self, user: NewAuthUser) -> AuthResult<AuthUser> {
        let now = OffsetDateTime::now_utc();
        let now_chrono = time_to_chrono(now)?;
        let email_verified = time_to_chrono_option(user.email_verified)?;

        let created = self
            .db
            .client
            .users
            .create(|u| {
                u.set_id(uuid::Uuid::new_v4().to_string())
                    .set_name(user.name)
                    .set_email(user.email)
                    .set_email_verified(email_verified)
                    .set_image(user.image)
                    .set_created_at(now_chrono)
                    .set_updated_at(now_chrono)
            })
            .await
            .map_err(|err| AuthError::Adapter(err.to_string()))?;

        map_user(created)
    }

    async fn link_account(&self, account: NewAuthAccount) -> AuthResult<AuthAccount> {
        let now = OffsetDateTime::now_utc();
        let now_chrono = time_to_chrono(now)?;
        let expires_at = time_to_chrono_option(account.expires_at)?;

        let created = self
            .db
            .client
            .accounts
            .create(|a| {
                a.set_id(uuid::Uuid::new_v4().to_string())
                    .set_user_id(account.user_id)
                    .set_provider(account.provider)
                    .set_provider_account_id(account.provider_account_id)
                    .set_access_token(account.access_token)
                    .set_refresh_token(account.refresh_token)
                    .set_expires_at(expires_at)
                    .set_token_type(account.token_type)
                    .set_scope(account.scope)
                    .set_id_token(account.id_token)
                    .set_created_at(now_chrono)
                    .set_updated_at(now_chrono)
            })
            .await
            .map_err(|err| AuthError::Adapter(err.to_string()))?;

        map_account(created)
    }

    async fn get_account(
        &self,
        provider: &str,
        provider_account_id: &str,
    ) -> AuthResult<Option<AuthAccount>> {
        let account = self
            .db
            .client
            .accounts
            .find_first(|a| {
                a.where_provider(provider.to_string())
                    .where_provider_account_id(provider_account_id.to_string())
            })
            .await
            .map_err(|err| AuthError::Adapter(err.to_string()))?;

        account.map(map_account).transpose()
    }

    async fn get_github_account_for_user(&self, user_id: &str) -> AuthResult<Option<AuthAccount>> {
        let account = self
            .db
            .client
            .accounts
            .find_first(|a| {
                a.where_user_id(user_id.to_string())
                    .where_provider("github".to_string())
            })
            .await
            .map_err(|err| AuthError::Adapter(err.to_string()))?;

        account.map(map_account).transpose()
    }
}

fn map_user(user: byteorm_client::Users) -> AuthResult<AuthUser> {
    Ok(AuthUser {
        id: user.id,
        name: user.name,
        email: user.email,
        email_verified: chrono_to_time_option(user.email_verified)?,
        image: user.image,
        created_at: chrono_to_time(user.created_at)?,
        updated_at: chrono_to_time(user.updated_at)?,
    })
}

fn map_account(account: byteorm_client::Accounts) -> AuthResult<AuthAccount> {
    Ok(AuthAccount {
        id: account.id,
        user_id: account.user_id,
        provider: account.provider,
        provider_account_id: account.provider_account_id,
        access_token: account.access_token,
        refresh_token: account.refresh_token,
        expires_at: chrono_to_time_option(account.expires_at)?,
        token_type: account.token_type,
        scope: account.scope,
        id_token: account.id_token,
        created_at: chrono_to_time(account.created_at)?,
        updated_at: chrono_to_time(account.updated_at)?,
    })
}
