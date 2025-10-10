use anyhow::Result;
use serde::Deserialize;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

use crate::args::Args;

#[derive(Clone, Deserialize)]
#[allow(dead_code)]
pub struct AppConfig {
    pub base_url: String,
}

#[derive(Clone, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
}

#[derive(Clone, Deserialize)]
pub struct FrontendConfig {
    pub base_url: String,
}

#[derive(Clone, Deserialize)]
pub struct ServerConfig {
    pub bind: String,
    pub port: u16,
}

#[derive(Clone, Deserialize)]
#[allow(dead_code)]
pub struct ApplicationConfig {
    pub app: AppConfig,
    pub database: DatabaseConfig,
    pub frontend: FrontendConfig,
    pub server: ServerConfig,
}

impl ApplicationConfig {
    pub async fn load(args: &Args) -> Result<Self> {
        let mut file = File::open(&args.config_file).await?;
        let mut content = String::new();
        file.read_to_string(&mut content).await?;
        let config = toml::from_str(&content)?;
        Ok(config)
    }
}
