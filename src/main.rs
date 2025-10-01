use actix_cors::Cors;
use actix_session::{SessionMiddleware, storage::CookieSessionStore};
use actix_web::cookie::Key;
use actix_web::http::header;
use actix_web::middleware::Logger;
use actix_web::web::{scope, Data};
use actix_web::{App, HttpServer};
use env_logger::Env;

mod args;
mod config;
mod context;
mod errors;
mod handlers;
mod middlewares;
mod models;
mod secrets;

use self::config::ApplicationConfig;
use self::context::ApplicationContext;
use self::middlewares::require_api_key::RequireApiKey;
use self::secrets::Secrets;

fn build_cors(config: &ApplicationConfig) -> Cors {
    Cors::default()
        .allowed_origin(&config.frontend.base_url)
        .allowed_methods(vec!["POST", "GET", "DELETE", "OPTIONS"])
        .allowed_headers(vec![header::CONTENT_TYPE])
        .supports_credentials()
}

fn build_session_middleware() -> SessionMiddleware<CookieSessionStore> {
    let session_key = Secrets::default().session.session_key();
    SessionMiddleware::new(CookieSessionStore::default(), Key::from(&session_key))
}

#[actix_rt::main]
async fn main() -> anyhow::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let args = args::parse();
    let config = ApplicationConfig::load(&args).await?;
    let context = ApplicationContext::new(&config)?;
    let server_config = config.server.clone();

    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::new("%a %t \"%r\" %s %b \"%{Referer}i\" \"%{User-Agent}i\" %T"))
            .wrap(build_cors(&config))
            .wrap(build_session_middleware())
            .app_data(Data::new(context.clone()))
            .service(
                scope("/api")
                    .wrap(RequireApiKey::new())
                    .configure(handlers::api::routes)
            )
    });
    server.bind((server_config.bind, server_config.port))?.run().await?;
    Ok(())
}
