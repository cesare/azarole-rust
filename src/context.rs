use anyhow::Result;
use sqlx::{Pool, Sqlite, sqlite::SqlitePoolOptions};

use crate::{config::ApplicationConfig, repositories::RdbRepository, secrets::Secrets};

#[derive(Clone)]
pub struct DatabaseContext {
    pub pool: Pool<Sqlite>,
}

impl DatabaseContext {
    fn new(config: &ApplicationConfig) -> Result<Self> {
        let pool = SqlitePoolOptions::new().connect_lazy(&config.database.url)?;
        Ok(Self { pool })
    }
}

#[derive(Clone)]
#[allow(dead_code)]
pub struct AppState {
    pub config: ApplicationConfig,
    pub database: DatabaseContext,
    pub repository: RdbRepository,
    pub secrets: Secrets,
}

impl AppState {
    pub fn new(config: &ApplicationConfig) -> Result<Self> {
        let database = DatabaseContext::new(config)?;
        let repository = RdbRepository::new(database.pool.clone());
        let secrets = Secrets::load()?;
        let app_state = Self {
            config: config.clone(),
            repository,
            secrets,
            database,
        };
        Ok(app_state)
    }
}
