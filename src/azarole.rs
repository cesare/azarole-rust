use actix_web::{App, HttpServer};

mod app;
use self::app::args;

#[actix_rt::main]
async fn main() -> anyhow::Result<()> {
    let args = args::parse();
    let config = app::config::load(&args.config_file).await?;

    let server = HttpServer::new(move || {
        App::new()
    });
    server.bind((config.server.bind, config.server.port))?.run().await?;
    Ok(())
}
