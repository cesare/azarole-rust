use serde::{Deserialize, Serialize};

use super::{AuthError, RedirectUri};
use crate::AppState;

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

pub(super) struct AccessTokenRequest<'a> {
    app_state: &'a AppState,
}

impl<'a> AccessTokenRequest<'a> {
    pub(super) fn new(app_state: &'a AppState) -> Self {
        Self { app_state }
    }

    pub(super) async fn execute(&self, code: &str) -> Result<AccessTokenResponse, AuthError> {
        let parameters = Parameters {
            client_id: &self.app_state.secrets.google_auth.client_id,
            client_secret: &self.app_state.secrets.google_auth.client_secret,
            code,
            grant_type: "authorization_code",
            redirect_uri: &RedirectUri::new(&self.app_state.config),
        };

        let client = reqwest::Client::new();
        let raw_response = client
            .post("https://oauth2.googleapis.com/token")
            .form(&parameters)
            .send()
            .await
            .inspect_err(|e| log::error!("Access token request failed: {:?}", e))?;

        let response = raw_response
            .json::<AccessTokenResponse>()
            .await
            .inspect_err(|e| log::error!("Failed to parse access token response: {:?}", e))?;
        Ok(response)
    }
}
