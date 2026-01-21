use anyhow::Result;

use crate::context::ApplicationContext;
use crate::models::{TokenDigester, User};
use crate::repositories::RepositoryFactory;

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
        let repository = self.context.repositories.api_key();
        match repository.find_by_digest(&digest).await? {
            Some(api_key) => Ok(Some(User::new(api_key.user_id))),
            _ => Ok(None),
        }
    }

    fn digest_token(&self) -> Result<String> {
        let digester = TokenDigester::new(&self.context.secrets.api_key.digesting_secret_key);
        digester.digest_token(self.token)
    }
}
