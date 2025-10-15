use jsonwebtoken::{
    decode, decode_header, jwk::{Jwk, JwkSet}, Algorithm, DecodingKey, Validation
};
use serde::Deserialize;

use crate::secrets::Secrets;

use super::AuthError;

#[derive(Clone, Deserialize)]
#[allow(dead_code)]
pub(super) struct Claims {
    aud: String,
    exp: i64,
    iss: String,
    nonce: String,
    pub(super) sub: String,
}

pub(super) struct IdTokenVerifier<'a> {
    token: &'a str,
    nonce: &'a str,
}

impl<'a> IdTokenVerifier<'a> {
    pub(super) fn new(token: &'a str, nonce: &'a str) -> Self {
        Self { token, nonce }
    }

    pub(super) async fn verify(self) -> Result<Claims, AuthError> {
        let key_id = self.find_key_id()?;
        let jwks = self.fetch_jwks().await?;
        match jwks.find(&key_id) {
            Some(jwk) => self.verify_id_token(jwk),
            None => Err(AuthError::InvalidIdToken),
        }
    }

    async fn fetch_jwks(&self) -> Result<JwkSet, AuthError> {
        let client = reqwest::Client::new();
        let response = client.get("https://www.googleapis.com/oauth2/v3/certs").send().await?;
        let jwks = response.json::<JwkSet>().await?;
        Ok(jwks)
    }

    fn find_key_id(&self) -> Result<String, AuthError> {
        let header = decode_header(self.token)?;
        match header.kid {
            Some(kid) => Ok(kid.to_owned()),
            _ => Err(AuthError::InvalidIdToken)
        }
    }

    fn verify_id_token(&self, jwk: &Jwk) -> Result<Claims, AuthError> {
        let decoding_key = DecodingKey::from_jwk(jwk)?;

        let secrets = Secrets::default();
        let client_id = secrets.google_auth.client_id();

        let mut validation = Validation::new(Algorithm::RS256);
        validation.set_audience(&[client_id]);

        let jwt = decode::<Claims>(&self.token, &decoding_key, &validation)?;
        let claims = jwt.claims;

        if claims.nonce != self.nonce {
            return Err(AuthError::InvalidIdToken);
        }

        Ok(claims)
    }
}
