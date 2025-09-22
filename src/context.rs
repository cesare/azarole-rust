use anyhow::Result;
use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite};

use crate::config::ApplicationConfig;

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
pub struct ApplicationContext {
    pub config: ApplicationConfig,
    pub database: DatabaseContext,
}

impl ApplicationContext {
    pub fn new(config: &ApplicationConfig) -> Result<Self> {
        let database = DatabaseContext::new(config)?;
        let context = Self {
            config: config.clone(),
            database
        };
        Ok(context)
    }
}
