use std::path::PathBuf;

use actix_web::{App, HttpServer};
use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    config_file: PathBuf,
}

#[actix_rt::main]
async fn main() -> anyhow::Result<()> {
    let _args = Args::parse();

    let server = HttpServer::new(move || {
        App::new()
    });
    server.bind(("127.0.0.1", 3000))?.run().await?;
    Ok(())
}
