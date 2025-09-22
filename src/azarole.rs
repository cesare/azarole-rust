use actix_web::{App, HttpServer};

mod app;
use self::app::args;
use self::app::config::ApplicationConfig;

#[actix_rt::main]
async fn main() -> anyhow::Result<()> {
    let args = args::parse();
    let config = ApplicationConfig::load(&args).await?;

    let server = HttpServer::new(move || {
        App::new()
    });
    server.bind((config.server.bind, config.server.port))?.run().await?;
    Ok(())
}
