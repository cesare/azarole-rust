use std::sync::Arc;

use serde::{Deserialize, Serialize};

use crate::{
    context::ApplicationContext,
    secrets::Secrets,
};
use super::{AuthError, RedirectUri};

#[derive(Serialize)]
struct Parameters<'a> {
    client_id: &'a str,
    client_secret: &'a str,
    code: &'a str,
    grant_type: &'a str,
    redirect_uri: &'a RedirectUri,
}

#[derive(Deserialize)]
#[allow(dead_code)]
pub(super) struct AccessTokenResponse {
    access_token: String,
    pub(super) id_token: String,
    expires_in: u32,
    scope: String,
    token_type: String,
}

pub(super) struct AccessTokenRequest {
    context: Arc<ApplicationContext>,
}

impl AccessTokenRequest {
    pub(super) fn new(context: Arc<ApplicationContext>) -> Self {
        Self { context }
    }

    pub(super) async fn execute(&self, code: &str) -> Result<AccessTokenResponse, AuthError> {
        let secrets = Secrets::default();
        let parameters = Parameters {
            client_id: &secrets.google_auth.client_id(),
            client_secret: &secrets.google_auth.client_secret(),
            code,
            grant_type: "authorization_code",
            redirect_uri: &RedirectUri::new(&self.context.config),
        };

        let client = reqwest::Client::new();
        let raw_response = client.post("https://oauth2.googleapis.com/token")
            .form(&parameters)
            .send()
            .await?;
        let response = raw_response.json::<AccessTokenResponse>().await?;
        Ok(response)
    }
}
