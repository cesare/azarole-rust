use base64::{engine::general_purpose::URL_SAFE, Engine};
use rand::{
    Rng as _,
    SeedableRng as _,
    rngs::StdRng
};
use url::Url;

use crate::{
    config::ApplicationConfig,
    secrets::Secrets,
};
use super::RedirectUri;

pub(super) struct AuthenticationRequest {
    pub(super) state: String,
    pub(super) nonce: String,
    pub(super) request_url: String,
}

pub(super) struct AuthenticationRequestGenerator<'a> {
    config: &'a ApplicationConfig,
}

impl<'a> AuthenticationRequestGenerator<'a> {
    pub(super) fn new(config: &'a ApplicationConfig) -> Self {
        Self { config }
    }

    pub(super) fn generate(&self) -> AuthenticationRequest {
        let state = self.generate_random_string();
        let nonce = self.generate_random_string();
        let request_url = self.build_request_url(&state, &nonce);

        AuthenticationRequest { state, nonce, request_url }
    }

    fn build_request_url(&self, state: &str, nonce: &str) -> String {
        let secrets = Secrets::default();
        let client_id = secrets.google_auth.client_id();
        let redirect_uri = RedirectUri::new(&self.config);

        let url = Url::parse_with_params("https://accounts.google.com/o/oauth2/v2/auth", &[
            ("client_id", client_id),
            ("redirect_uri", redirect_uri.into()),
            ("response_type", "code".to_owned()),
            ("scope", "openid email".to_owned()),
            ("state", state.to_owned()),
            ("nonce", nonce.to_owned()),
        ]).unwrap();
        url.into()
    }

    fn generate_random_string(&self) -> String {
        let mut rng = StdRng::from_os_rng();
        let mut bytes = [0u8; 36];
        rng.fill(&mut bytes[..]);

        URL_SAFE.encode(bytes)
    }
}
