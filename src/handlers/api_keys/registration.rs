use base64::{engine::general_purpose::URL_SAFE, Engine};
use chrono::Utc;
use hmac::{Hmac, Mac};
use rand::{
    Rng as _,
    SeedableRng as _,
    rngs::StdRng
};
use serde::Serialize;
use sha2::Sha256;

use crate::{
    context::ApplicationContext,
    errors::DatabaseError,
    models::{ApiKey, ApiKeyId, User},
    secrets::Secrets,
};

pub(super) struct ApiKeyRegistration<'a> {
    context: &'a ApplicationContext,
    user: &'a User,
    name: &'a str,
}

#[derive(Serialize)]
pub(super) struct RegistationDetails {
    id: ApiKeyId,
    name: String,
    token: String,
}

impl<'a> ApiKeyRegistration<'a> {
    pub(super) fn new(context: &'a ApplicationContext, user: &'a User, name: &'a str) -> Self {
        Self { context, user, name }
    }

    pub(super) async fn execute(self) -> Result<RegistationDetails, DatabaseError> {
        let token = self.generate_token();
        let digest = self.digest_token(&token);
        let api_key = self.save(&digest).await?;

        let details = RegistationDetails {
            id: api_key.id,
            name: api_key.name,
            token,
        };
        Ok(details)
    }

    fn generate_token(&self) -> String {
        let mut rng = StdRng::from_os_rng();
        let mut bytes = [0u8; 96];
        rng.fill(&mut bytes[..]);

        URL_SAFE.encode(bytes)
    }

    fn digest_token(&self, token: &str) -> String {
        let secrets = Secrets::default();
        let secret_key = secrets.api_key.digesting_secret_key();
        let mut mac = Hmac::<Sha256>::new_from_slice(secret_key.as_bytes()).unwrap();
        mac.update(token.as_bytes());
        let result = mac.finalize();
        let bytes = result.into_bytes();
        hex::encode(bytes)
    }

    async fn save(&self, digest: &str) -> Result<ApiKey, DatabaseError> {
        let statement = "insert into api_keys (user_id, name, digest, created_at) values ($1, $2, $3, $4) returning id, user_id, name, digest, created_at";
        let now = Utc::now();
        let api_key: ApiKey = sqlx::query_as(statement)
            .bind(self.user.id)
            .bind(self.name)
            .bind(digest)
            .bind(now)
            .fetch_one(&self.context.database.pool)
            .await?;
        Ok(api_key)
    }
}
