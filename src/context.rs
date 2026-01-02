use anyhow::Result;
use sqlx::{Pool, Sqlite, sqlite::SqlitePoolOptions};

use crate::{config::ApplicationConfig, secrets::Secrets};

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
pub struct ApplicationContext {
    pub config: ApplicationConfig,
    pub database: DatabaseContext,
    pub secrets: Secrets,
}

impl ApplicationContext {
    pub fn new(config: &ApplicationConfig) -> Result<Self> {
        let database = DatabaseContext::new(config)?;
        let secrets = Secrets::load()?;
        let context = Self {
            config: config.clone(),
            secrets,
            database,
        };
        Ok(context)
    }
}
