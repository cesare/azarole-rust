use std::path::PathBuf;

use anyhow::Result;
use serde::Deserialize;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

#[derive(Deserialize)]
pub struct ServerConfig {
    pub bind: String,
    pub port: u16,
}

#[derive(Deserialize)]
pub struct ApplicationConfig {
    pub server: ServerConfig,
}

pub async fn load(path: &PathBuf) -> Result<ApplicationConfig> {
    let mut file = File::open(path).await?;
    let mut content = String::new();
    file.read_to_string(&mut content).await?;
    let config = toml::from_str(&content)?;
    Ok(config)
}
