use std::marker::PhantomData;

use async_trait::async_trait;
use chrono::Utc;
use sqlx::{Executor, Sqlite};

use super::ApiKeyRepository;
use crate::{
    errors::DatabaseError,
    models::{ApiKey, ApiKeyId, User},
};

pub struct RdbApiKeyRepository<'a, T: Executor<'a>> {
    executor: T,
    _marker: PhantomData<&'a T>,
}

impl<'a, T> RdbApiKeyRepository<'a, T>
where
    T: Executor<'a, Database = Sqlite> + Copy + Sync,
{
    pub fn new(executor: T) -> Self {
        Self {
            executor,
            _marker: PhantomData,
        }
    }
}

#[async_trait]
impl<'a, T> ApiKeyRepository for RdbApiKeyRepository<'a, T>
where
    T: Executor<'a, Database = Sqlite> + Copy + Sync,
{
    async fn list(&self, user: &User) -> Result<Vec<ApiKey>, DatabaseError> {
        let api_keys: Vec<ApiKey> = sqlx::query_as("select id, user_id, name, digest, created_at from api_keys where user_id = $1 order by created_at desc")
            .bind(user.id)
            .fetch_all(self.executor)
            .await
            .inspect_err(|e| log::error!("Failed to query api_keys: {:?}", e))?;

        Ok(api_keys)
    }

    async fn create(&self, user: &User, name: &str, digest: &str) -> Result<ApiKey, DatabaseError> {
        let statement = "insert into api_keys (user_id, name, digest, created_at) values ($1, $2, $3, $4) returning id, user_id, name, digest, created_at";
        let now = Utc::now();
        let api_key: ApiKey = sqlx::query_as(statement)
            .bind(user.id)
            .bind(name)
            .bind(digest)
            .bind(now)
            .fetch_one(self.executor)
            .await?;
        Ok(api_key)
    }

    async fn destroy(&self, user: &User, id: &ApiKeyId) -> Result<(), DatabaseError> {
        sqlx::query("delete from api_keys where user_id = $1 and id = $2")
            .bind(user.id)
            .bind(id)
            .execute(self.executor)
            .await
            .inspect_err(|e| log::error!("Failed to delete api_key: {:?}", e))?;
        Ok(())
    }
}
