use anyhow::Result;

use crate::context::ApplicationContext;
use crate::models::user::User;

pub struct ApiKeyAuthenticator {
    token: String,
}

impl ApiKeyAuthenticator {
    pub fn new(token: &str) -> Self {
        Self {
            token: token.to_owned(),
        }
    }

    pub async fn authenticate(&self, context: &ApplicationContext) -> Result<User> {
        Ok(User::new(1))
    }
}
