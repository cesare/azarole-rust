use anyhow::Result;
use sha2::Sha256;
use hmac::{Hmac, Mac};

use crate::context::ApplicationContext;
use crate::models::User;
use crate::models::ApiKey;

pub(super) struct ApiKeyAuthenticator<'a> {
    context: &'a ApplicationContext,
    token: &'a str,
}

impl<'a> ApiKeyAuthenticator<'a> {
    pub(super) fn new(context: &'a ApplicationContext, token: &'a str) -> Self {
        Self { context, token }
    }

    pub(super) async fn authenticate(self) -> Result<Option<User>> {
        let digest = self.digest_token()?;
        match self.find_api_token(&digest).await? {
            Some(api_key) => {
                Ok(Some(User::new(api_key.user_id)))
            },
            _ => {
                Ok(None)
            }
        }
    }

    fn digest_token(&self) -> Result<String> {
        let secret_key = &self.context.secrets.api_key.digesting_secret_key;
        let mut mac = Hmac::<Sha256>::new_from_slice(secret_key.as_bytes())
            .inspect_err(|e| log::error!("Failed to prepare Hmac object: {:?}", e))?;

        mac.update(self.token.as_bytes());
        let result = mac.finalize();
        let bytes = result.into_bytes();
        let digest = hex::encode(bytes);
        Ok(digest)
    }

    async fn find_api_token(&self, digest: &str) -> Result<Option<ApiKey>> {
        let result: Option<ApiKey> = sqlx::query_as("select id, user_id, name, digest, created_at from api_keys where digest = $1")
            .bind(digest)
            .fetch_optional(&self.context.database.pool)
            .await
            .inspect_err(|e| log::error!("Failed to query api_keys: {:?}", e))?;

        Ok(result)
    }
}
