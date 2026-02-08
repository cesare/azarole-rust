use jsonwebtoken::jwk::JwkSet;

use super::AuthError;

#[derive(Default)]
pub struct GoogleJwks;

impl GoogleJwks {
    pub async fn fetch(&self) -> Result<JwkSet, AuthError> {
        let client = reqwest::Client::new();
        let response = client
            .get("https://www.googleapis.com/oauth2/v3/certs")
            .send()
            .await
            .inspect_err(|e| log::error!("Failed to fetch google jwks: {:?}", e))?;

        let jwks = response
            .json::<JwkSet>()
            .await
            .inspect_err(|e| log::error!("Failed to parse google jwks: {:?}", e))?;

        Ok(jwks)
    }
}
