use std::env;
use anyhow::Result;
use base64::{engine::general_purpose::STANDARD, Engine as _};
use serde::{de, Deserialize};

#[derive(Clone)]
struct Base64Encoded(Vec<u8>);

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

#[derive(Clone, Default)]
pub struct ApikeyConfig;

impl ApikeyConfig {
    pub fn digesting_secret_key(&self) -> String {
        env::var("API_KEY_DIGESTING_SECRET_KEY").unwrap()
    }
}

#[derive(Clone, Default)]
pub struct GoogleAuthConfig;

impl GoogleAuthConfig {
    pub fn client_id(&self) -> String {
        env::var("GOOGLE_AUTH_CLIENT_ID").unwrap()
    }

    pub fn client_secret(&self) -> String {
        env::var("GOOGLE_AUTH_CLIENT_SECRET").unwrap()
    }
}

#[derive(Clone, Default)]
pub struct SessionConfig;

impl SessionConfig {
    pub fn session_key(&self) -> Vec<u8> {
        let base64_value = env::var("SESSION_KEY").unwrap();
        STANDARD.decode(base64_value).unwrap()
    }
}

#[derive(Clone, Default)]
pub struct Secrets {
    pub api_key: ApikeyConfig,
    pub google_auth: GoogleAuthConfig,
    pub session: SessionConfig,
}

impl Secrets {
    pub fn load() -> Result<Self> {
        Ok(Self::default())
    }
}
