use actix_web::middleware::Logger;
use actix_web::web::{scope, Data};
use actix_web::{App, HttpServer};
use env_logger::Env;

mod args;
mod config;
mod context;
mod handlers;
mod models;

use self::config::ApplicationConfig;
use self::context::ApplicationContext;

#[actix_rt::main]
async fn main() -> anyhow::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let args = args::parse();
    let config = ApplicationConfig::load(&args).await?;
    let context = ApplicationContext::new(&config)?;

    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::new("%a %t \"%r\" %s %b \"%{Referer}i\" \"%{User-Agent}i\" %T"))
            .app_data(Data::new(context.clone()))
            .service(
                scope("/api").configure(handlers::api::routes)
            )
    });
    server.bind((config.server.bind, config.server.port))?.run().await?;
    Ok(())
}
