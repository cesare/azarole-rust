use anyhow::Result;
use sha2::Sha256;
use hmac::{Hmac, Mac};

use crate::context::ApplicationContext;
use crate::models::user::User;
use crate::models::api_key::ApiKey;

pub struct ApiKeyAuthenticator {
    token: String,
}

impl ApiKeyAuthenticator {
    pub fn new(token: &str) -> Self {
        Self {
            token: token.to_owned(),
        }
    }

    pub async fn authenticate(&self, context: &ApplicationContext) -> Result<Option<User>> {
        let digest = self.digest_token()?;
        match self.find_api_token(&digest, context).await? {
            Some(api_key) => {
                Ok(Some(User::new(api_key.user_id)))
            },
            _ => {
                Ok(None)
            }
        }
    }

    fn digest_token(&self) -> Result<String> {
        let secret_key: String = std::env::var("API_KEY_DIGESTING_SECRET_KEY").unwrap();
        let mut mac = Hmac::<Sha256>::new_from_slice(secret_key.as_bytes())?;
        mac.update(&self.token.as_bytes());
        let result = mac.finalize();
        let bytes = result.into_bytes();
        let digest = hex::encode(bytes);
        Ok(digest)
    }

    async fn find_api_token(&self, digest: &str, context: &ApplicationContext) -> Result<Option<ApiKey>> {
        let result: Option<ApiKey> = sqlx::query_as("select id, user_id, name, digest from api_keys where digest = $1")
            .bind(digest)
            .fetch_optional(&context.database.pool)
            .await?;
        Ok(result)
    }
}
