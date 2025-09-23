use actix_web::web::{scope, Data};
use actix_web::{App, HttpServer};

mod args;
mod config;
mod context;
mod handlers;

use self::config::ApplicationConfig;
use self::context::ApplicationContext;

#[actix_rt::main]
async fn main() -> anyhow::Result<()> {
    let args = args::parse();
    let config = ApplicationConfig::load(&args).await?;
    let context = ApplicationContext::new(&config)?;

    let server = HttpServer::new(move || {
        App::new()
            .app_data(Data::new(context.clone()))
            .service(
                scope("/api").configure(handlers::api::routes)
            )
    });
    server.bind((config.server.bind, config.server.port))?.run().await?;
    Ok(())
}
