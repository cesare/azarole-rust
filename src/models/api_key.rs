use hmac::{Hmac, Mac};
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use sqlx::prelude::FromRow;

use super::{IdType, Timestamp, UserId};

#[derive(Clone, Copy, Deserialize, Serialize, sqlx::Type)]
#[sqlx(transparent)]
#[repr(transparent)]
pub struct ApiKeyId(IdType);

#[derive(Clone, Deserialize, FromRow, Serialize)]
#[allow(dead_code)]
pub struct ApiKey {
    pub id: ApiKeyId,
    pub user_id: UserId,
    pub name: String,
    pub digest: String,
    pub created_at: Timestamp,
}

pub struct TokenDigester {
    secret_key: Vec<u8>,
}

impl TokenDigester {
    pub fn new(secret_key: &[u8]) -> Self {
        Self {
            secret_key: secret_key.to_vec(),
        }
    }

    pub fn digest_token(&self, token: &str) -> anyhow::Result<String> {
        let mut mac = Hmac::<Sha256>::new_from_slice(&self.secret_key)
            .inspect_err(|e| log::error!("Failed to prepare Hmac object: {:?}", e))?;

        mac.update(token.as_bytes());
        let result = mac.finalize();
        let bytes = result.into_bytes();
        let digest = hex::encode(bytes);
        Ok(digest)
    }
}
