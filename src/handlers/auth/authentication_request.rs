use base64::{Engine, engine::general_purpose::URL_SAFE};
use rand::{Rng as _, SeedableRng as _, rngs::StdRng};
use url::Url;

use super::RedirectUri;
use crate::context::ApplicationContext;

pub(super) struct AuthenticationRequest {
    pub(super) state: String,
    pub(super) nonce: String,
    pub(super) request_url: String,
}

pub(super) struct AuthenticationRequestGenerator<'a> {
    context: &'a ApplicationContext,
}

impl<'a> AuthenticationRequestGenerator<'a> {
    pub(super) fn new(context: &'a ApplicationContext) -> Self {
        Self { context }
    }

    pub(super) fn generate(&self) -> AuthenticationRequest {
        let state = self.generate_random_string();
        let nonce = self.generate_random_string();
        let request_url = self.build_request_url(&state, &nonce);

        AuthenticationRequest {
            state,
            nonce,
            request_url,
        }
    }

    fn build_request_url(&self, state: &str, nonce: &str) -> String {
        let parameters: &[(&str, &str)] = &[
            ("client_id", &self.context.secrets.google_auth.client_id),
            ("redirect_uri", &RedirectUri::new(&self.context.config)),
            ("response_type", "code"),
            ("scope", "openid email"),
            ("state", state),
            ("nonce", nonce),
        ];
        Url::parse_with_params("https://accounts.google.com/o/oauth2/v2/auth", parameters)
            .unwrap()
            .into()
    }

    fn generate_random_string(&self) -> String {
        let mut rng = StdRng::from_os_rng();
        let mut bytes = [0u8; 36];
        rng.fill(&mut bytes[..]);

        URL_SAFE.encode(bytes)
    }
}
