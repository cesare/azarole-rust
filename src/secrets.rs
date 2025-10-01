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

pub struct Secrets {
    pub api_key: ApikeyConfig,
}

impl Secrets {
    pub fn default() -> Self {
        Self {
            api_key: ApikeyConfig::default(),
        }
    }
}
