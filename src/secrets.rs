use std::env;

pub struct ApikeyConfig;

impl ApikeyConfig {
    pub fn default() -> Self {
        Self {}
    }

    pub fn digesting_secret_key(&self) -> String {
        env::var("API_KEY_DIGESTING_SECRET_KEY").unwrap()
    }
}

pub struct GoogleAuthConfig;

impl GoogleAuthConfig {
    pub fn default() -> Self {
        Self {}
    }

    pub fn client_id(&self) -> String {
        env::var("GOOGLE_AUTH_CLIENT_ID").unwrap()
    }

    pub fn client_secret(&self) -> String {
        env::var("GOOGLE_AUTH_CLIENT_SECRET").unwrap()
    }
}

pub struct SessionConfig;

impl SessionConfig {
    pub fn default() -> Self {
        Self {}
    }
    pub fn session_key(&self) -> String {
        env::var("SESSION_KEY").unwrap()
    }
}

pub struct Secrets {
    pub api_key: ApikeyConfig,
    pub google_auth: GoogleAuthConfig,
    pub session: SessionConfig,
}

impl Secrets {
    pub fn default() -> Self {
        Self {
            api_key: ApikeyConfig::default(),
            google_auth: GoogleAuthConfig::default(),
            session: SessionConfig::default(),
        }
    }
}
