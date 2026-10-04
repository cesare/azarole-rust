use async_trait::async_trait;
use chrono::Utc;
use sqlx::{Executor, Pool, Sqlite};

use super::ApiKeyRepository;
use crate::{
    errors::DatabaseError,
    models::{ApiKey, ApiKeyId, User},
};

pub struct RdbApiKeyRepository<'r> {
    pool: &'r Pool<Sqlite>,
}

impl<'r> RdbApiKeyRepository<'r> {
    pub fn new(pool: &'r Pool<Sqlite>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl<'r> ApiKeyRepository for RdbApiKeyRepository<'r> {
    async fn list(&self, user: &User) -> Result<Vec<ApiKey>, DatabaseError> {
        list(self.pool, user).await
    }

    async fn find_by_digest(&self, digest: &str) -> Result<Option<ApiKey>, DatabaseError> {
        find_by_digest(self.pool, digest).await
    }

    async fn create(&self, user: &User, name: &str, digest: &str) -> Result<ApiKey, DatabaseError> {
        create(self.pool, user, name, digest).await
    }

    async fn destroy(&self, user: &User, id: &ApiKeyId) -> Result<(), DatabaseError> {
        destroy(self.pool, user, id).await
    }
}

async fn list<'c, T>(executor: T, user: &User) -> Result<Vec<ApiKey>, DatabaseError>
where
    T: Executor<'c, Database = Sqlite>,
{
    let api_keys: Vec<ApiKey> = sqlx::query_as("select id, user_id, name, digest, created_at from api_keys where user_id = $1 order by created_at desc")
        .bind(user.id)
        .fetch_all(executor)
        .await
        .inspect_err(|e| log::error!("Failed to query api_keys: {:?}", e))?;

    Ok(api_keys)
}

async fn find_by_digest<'c, T>(executor: T, digest: &str) -> Result<Option<ApiKey>, DatabaseError>
where
    T: Executor<'c, Database = Sqlite>,
{
    let statement = "select id, user_id, name, digest, created_at from api_keys where digest = $1";
    let result: Option<ApiKey> = sqlx::query_as(statement)
        .bind(digest)
        .fetch_optional(executor)
        .await
        .inspect_err(|e| log::error!("Failed to query api_keys: {:?}", e))?;

    Ok(result)
}

async fn create<'c, T>(
    executor: T,
    user: &User,
    name: &str,
    digest: &str,
) -> Result<ApiKey, DatabaseError>
where
    T: Executor<'c, Database = Sqlite>,
{
    let statement = "insert into api_keys (user_id, name, digest, created_at) values ($1, $2, $3, $4) returning id, user_id, name, digest, created_at";
    let now = Utc::now();
    let api_key: ApiKey = sqlx::query_as(statement)
        .bind(user.id)
        .bind(name)
        .bind(digest)
        .bind(now)
        .fetch_one(executor)
        .await?;
    Ok(api_key)
}

async fn destroy<'c, T>(executor: T, user: &User, id: &ApiKeyId) -> Result<(), DatabaseError>
where
    T: Executor<'c, Database = Sqlite>,
{
    sqlx::query("delete from api_keys where user_id = $1 and id = $2")
        .bind(user.id)
        .bind(id)
        .execute(executor)
        .await
        .inspect_err(|e| log::error!("Failed to delete api_key: {:?}", e))?;
    Ok(())
}
