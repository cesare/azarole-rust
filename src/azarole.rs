use std::path::PathBuf;

use actix_web::{App, HttpServer};
use clap::Parser;

mod app;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    config_file: PathBuf,
}

#[actix_rt::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let config = app::config::load(&args.config_file).await?;

    let server = HttpServer::new(move || {
        App::new()
    });
    server.bind((config.server.bind, config.server.port))?.run().await?;
    Ok(())
}
