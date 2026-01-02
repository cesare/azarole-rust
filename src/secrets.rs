use anyhow::Result;
use base64::{Engine as _, engine::general_purpose::STANDARD};
use serde::{Deserialize, de};
use std::ops::Deref;

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
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_string(Base64EncodedVisitor)
    }
}

struct Base64EncodedVisitor;
impl<'de> de::Visitor<'de> for Base64EncodedVisitor {
    type Value = Base64Encoded;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a base64 encoded string")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        STANDARD.decode(v).map(Base64Encoded).map_err(E::custom)
    }
}

#[derive(Clone, Deserialize)]
pub struct ApikeyConfig {
    #[serde(rename = "api_key_digesting_secret_key")]
    pub digesting_secret_key: Base64Encoded,
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
        let api_key = envy::from_env::<ApikeyConfig>()?;
        let google_auth = envy::from_env::<GoogleAuthConfig>()?;
        let session = envy::from_env::<SessionConfig>()?;

        let secrets = Self {
            api_key,
            google_auth,
            session,
        };
        Ok(secrets)
    }
}
