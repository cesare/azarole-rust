use std::sync::Arc;

use serde::{Deserialize, Serialize};

use crate::{
    context::ApplicationContext,
    secrets::Secrets,
};
use super::{AuthError, RedirectUri};

#[derive(Serialize)]
struct Parameters {
    client_id: String,
    client_secret: String,
    code: String,
    grant_type: String,
    redirect_uri: RedirectUri,
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
        let client_id = secrets.google_auth.client_id().clone();
        let client_secret = secrets.google_auth.client_secret().clone();
        let redirect_uri = RedirectUri::new(&self.context.config);

        let parameters = Parameters {
            client_id,
            client_secret,
            code: code.to_owned(),
            grant_type: "authorization_code".to_owned(),
            redirect_uri,
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
