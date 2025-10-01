use std::env;

#[derive(Default)]
pub struct ApikeyConfig;

impl ApikeyConfig {
    pub fn digesting_secret_key(&self) -> String {
        env::var("API_KEY_DIGESTING_SECRET_KEY").unwrap()
    }
}

#[derive(Default)]
pub struct GoogleAuthConfig;

impl GoogleAuthConfig {
    pub fn client_id(&self) -> String {
        env::var("GOOGLE_AUTH_CLIENT_ID").unwrap()
    }

    pub fn client_secret(&self) -> String {
        env::var("GOOGLE_AUTH_CLIENT_SECRET").unwrap()
    }
}

#[derive(Default)]
pub struct SessionConfig;

impl SessionConfig {
    pub fn session_key(&self) -> String {
        env::var("SESSION_KEY").unwrap()
    }
}

#[derive(Default)]
pub struct Secrets {
    pub api_key: ApikeyConfig,
    pub google_auth: GoogleAuthConfig,
    pub session: SessionConfig,
}
