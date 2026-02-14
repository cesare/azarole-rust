use base64::{Engine, engine::general_purpose::URL_SAFE};
use serde::Serialize;

use crate::{
    context::AppState,
    errors::DatabaseError,
    models::{ApiKeyId, TokenDigester, TokenGenerator, User},
    repositories::RepositoryFactory,
};

pub(super) struct ApiKeyRegistration<'a> {
    context: &'a AppState,
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
    pub(super) fn new(context: &'a AppState, user: &'a User, name: &'a str) -> Self {
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
        let raw_token = TokenGenerator.generate();
        URL_SAFE.encode(&raw_token)
    }

    fn digest_token(&self, token: &str) -> String {
        let digester = TokenDigester::new(&self.context.secrets.api_key.digesting_secret_key);
        digester.digest_token(token).unwrap()
    }
}
