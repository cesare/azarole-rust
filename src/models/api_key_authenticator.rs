use anyhow::Result;

use crate::context::ApplicationContext;
use crate::models::user::User;

pub struct ApiKeyAuthenticator {
    context: ApplicationContext,
    token: String,
}

impl ApiKeyAuthenticator {
    pub fn new(context: ApplicationContext, token: &str) -> Self {
        Self {
            context,
            token: token.to_owned(),
        }
    }

    pub async fn authenticate(&self) -> Result<User> {
        Ok(User::new(1))
    }
}
