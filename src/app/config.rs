use anyhow::Result;
use serde::Deserialize;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

use super::args::Args;

#[derive(Deserialize)]
pub struct ServerConfig {
    pub bind: String,
    pub port: u16,
}

#[derive(Deserialize)]
pub struct ApplicationConfig {
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
