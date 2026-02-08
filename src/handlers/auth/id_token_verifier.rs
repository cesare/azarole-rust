use jsonwebtoken::{Algorithm, DecodingKey, Validation, decode, decode_header, jwk::Jwk};
use serde::Deserialize;

use crate::{context::ApplicationContext, handlers::auth::google_jwks::GoogleJwks};

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
    context: &'a ApplicationContext,
    token: &'a str,
    nonce: &'a str,
    jwks: GoogleJwks,
}

impl<'a> IdTokenVerifier<'a> {
    pub(super) fn new(context: &'a ApplicationContext, token: &'a str, nonce: &'a str) -> Self {
        let jwks = GoogleJwks::default();

        Self {
            context,
            token,
            nonce,
            jwks,
        }
    }

    pub(super) async fn verify(self) -> Result<Claims, AuthError> {
        let key_id = self.find_key_id(self.token)?;
        let jwks = self.jwks.fetch().await?;
        match jwks.find(&key_id) {
            Some(jwk) => self.verify_id_token(jwk, self.token, self.nonce),
            None => {
                log::error!("Failed to find key_id {} in jwk", key_id);
                Err(AuthError::InvalidIdToken)
            }
        }
    }

    fn find_key_id(&self, token: &str) -> Result<String, AuthError> {
        let header = decode_header(token)
            .inspect_err(|e| log::error!("Failed to decode token header: {:?}", e))?;

        match header.kid {
            Some(kid) => Ok(kid.to_owned()),
            _ => {
                log::error!("kid missing in token header");
                Err(AuthError::InvalidIdToken)
            }
        }
    }

    fn verify_id_token(&self, jwk: &Jwk, token: &str, nonce: &str) -> Result<Claims, AuthError> {
        let decoding_key = DecodingKey::from_jwk(jwk)
            .inspect_err(|e| log::error!("Failed to detect decoding key from jwk: {:?}", e))?;

        let client_id = &self.context.secrets.google_auth.client_id;

        let mut validation = Validation::new(Algorithm::RS256);
        validation.set_audience(&[client_id]);

        let jwt = decode::<Claims>(token, &decoding_key, &validation)
            .inspect_err(|e| log::error!("Failed to decode claims from token: {:?}", e))?;

        let claims = jwt.claims;

        if claims.nonce != nonce {
            return Err(AuthError::InvalidIdToken);
        }

        Ok(claims)
    }
}
