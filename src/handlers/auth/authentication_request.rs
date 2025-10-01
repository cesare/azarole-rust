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

pub(super) struct AuthenticationRequest {
    pub(super) state: String,
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
        let state = self.generate_state();
        let request_url = self.build_request_url(&state);

        AuthenticationRequest { state, request_url }
    }

    fn build_request_url(&self, state: &str) -> String {
        let secrets = Secrets::default();
        let client_id = secrets.google_auth.client_id();
        let redirect_uri = format!("{}/auth/google/callback", self.config.app.base_url);

        let url = Url::parse_with_params("https://accounts.google.com/o/oauth2/v2/auth", &[
            ("client_id", client_id),
            ("redirect_uri", redirect_uri),
            ("response_type", "code".to_owned()),
            ("scope", "openid".to_owned()),
            ("state", state.to_owned()),
        ]).unwrap();
        url.into()
    }

    fn generate_state(&self) -> String {
        let mut rng = StdRng::from_os_rng();
        let mut bytes = [0u8; 120];
        rng.fill(&mut bytes[..]);

        URL_SAFE.encode(bytes)
    }
}
