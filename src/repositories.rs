use async_trait::async_trait;
use sqlx::{Pool, Sqlite};

use crate::{
    errors::DatabaseError,
    models::{ApiKey, ApiKeyId, User},
    repositories::api_key::RdbApiKeyRepository,
};

mod api_key;

#[async_trait]
pub trait ApiKeyRepository {
    async fn list(&self, user: &User) -> Result<Vec<ApiKey>, DatabaseError>;
    async fn destroy(&self, user: &User, id: &ApiKeyId) -> Result<(), DatabaseError>;
}

pub trait RepositoryFactory {
    fn api_key(&self) -> Box<dyn ApiKeyRepository + '_>;
}

#[derive(Clone)]
pub struct RdbRepositories {
    pool: Pool<Sqlite>,
}

impl RdbRepositories {
    pub fn new(pool: Pool<Sqlite>) -> Self {
        Self { pool }
    }
}

impl RepositoryFactory for RdbRepositories {
    fn api_key(&self) -> Box<dyn ApiKeyRepository + '_> {
        Box::new(RdbApiKeyRepository::new(&self.pool))
    }
}
