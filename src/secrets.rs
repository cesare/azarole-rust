use std::{env, ops::Deref};
use anyhow::Result;
use base64::{engine::general_purpose::STANDARD, Engine as _};
use serde::{de, Deserialize};

#[derive(Clone)]
pub struct Base64Encoded(Vec<u8>);

impl Deref for Base64Encoded {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'de> Deserialize<'de> for Base64Encoded {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {
        deserializer.deserialize_string(Base64EncodedVisitor)
    }
}

struct Base64EncodedVisitor;
impl<'de> de::Visitor<'de> for Base64EncodedVisitor {
    type Value = Base64Encoded;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a base64 encoded string")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: de::Error {
        STANDARD.decode(v).map(Base64Encoded).map_err(E::custom)
    }
}

#[derive(Clone)]
pub struct ApikeyConfig;

impl ApikeyConfig {
    pub fn digesting_secret_key(&self) -> String {
        env::var("API_KEY_DIGESTING_SECRET_KEY").unwrap()
    }
}

#[derive(Clone, Deserialize)]
pub struct GoogleAuthConfig {
    #[serde(rename = "google_auth_client_id")]
    pub client_id: String,

    #[serde(rename = "google_auth_client_secret")]
    pub client_secret: String,
}

#[derive(Clone, Deserialize)]
pub struct SessionConfig {
    pub session_key: Base64Encoded,
}

#[derive(Clone)]
pub struct Secrets {
    pub api_key: ApikeyConfig,
    pub google_auth: GoogleAuthConfig,
    pub session: SessionConfig,
}

impl Secrets {
    pub fn load() -> Result<Self> {
        let google_auth = envy::from_env::<GoogleAuthConfig>()?;
        let session = envy::from_env::<SessionConfig>()?;

        let secrets = Self {
            api_key: ApikeyConfig,
            google_auth,
            session,
        };
        Ok(secrets)
    }
}
