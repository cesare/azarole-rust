use base64::{Engine, engine::general_purpose::URL_SAFE};
use hmac::{Hmac, Mac};
use rand::{Rng as _, SeedableRng as _, rngs::StdRng};
use serde::Serialize;
use sha2::Sha256;

use crate::{
    context::ApplicationContext,
    errors::DatabaseError,
    models::{ApiKeyId, User},
    repositories::RepositoryFactory,
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
        Self {
            context,
            user,
            name,
        }
    }

    pub(super) async fn execute(self) -> Result<RegistationDetails, DatabaseError> {
        let token = self.generate_token();
        let digest = self.digest_token(&token);

        let repository = self.context.repositories.api_key();
        let api_key = repository.create(self.user, self.name, &digest).await?;

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
        let mut mac =
            Hmac::<Sha256>::new_from_slice(&self.context.secrets.api_key.digesting_secret_key)
                .unwrap();
        mac.update(token.as_bytes());
        let result = mac.finalize();
        let bytes = result.into_bytes();
        hex::encode(bytes)
    }
}
