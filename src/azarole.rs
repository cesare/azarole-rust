use actix_web::web::Data;
use actix_web::{App, HttpServer};

mod app;
use self::app::args;
use self::app::config::ApplicationConfig;
use self::app::context::ApplicationContext;

#[actix_rt::main]
async fn main() -> anyhow::Result<()> {
    let args = args::parse();
    let config = ApplicationConfig::load(&args).await?;
    let context = ApplicationContext::new(&config)?;

    let server = HttpServer::new(move || {
        App::new()
            .app_data(Data::new(context.clone()))
    });
    server.bind((config.server.bind, config.server.port))?.run().await?;
    Ok(())
}
