use std::path::PathBuf;

use actix_web::{App, HttpServer};
use clap::Parser;
use serde::Deserialize;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    config_file: PathBuf,
}

#[derive(Deserialize)]
struct ServerConfig {
    bind: String,
    port: u16,
}

#[derive(Deserialize)]
struct ApplicationConfig {
    server: ServerConfig,
}

#[actix_rt::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let mut file = File::open(&args.config_file).await?;
    let mut content = String::new();
    file.read_to_string(&mut content).await?;
    let config: ApplicationConfig = toml::from_str(&content)?;

    let server = HttpServer::new(move || {
        App::new()
    });
    server.bind((config.server.bind, config.server.port))?.run().await?;
    Ok(())
}
