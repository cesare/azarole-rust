use async_trait::async_trait;
use sqlx::{Executor, Pool, Sqlite, Transaction};

use crate::{
    errors::DatabaseError,
    models::{User, UserId},
    repositories::TransactionalUserRepository,
};

use super::UserRepository;

pub struct RdbUserRepository<'r> {
    pool: &'r Pool<Sqlite>,
}

impl<'r> RdbUserRepository<'r> {
    pub fn new(pool: &'r Pool<Sqlite>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl<'r> UserRepository for RdbUserRepository<'r> {
    async fn find_optional(&self, id: UserId) -> Result<Option<User>, DatabaseError> {
        find_optional(self.pool, id).await
    }
}

pub struct TxUserRepository<'r> {
    tx: &'r mut Transaction<'r, Sqlite>,
}

impl<'r> TxUserRepository<'r> {
    pub(super) fn new(tx: &'r mut Transaction<'r, Sqlite>) -> Self {
        TxUserRepository { tx }
    }
}

#[async_trait]
impl<'r> TransactionalUserRepository for TxUserRepository<'r> {
    async fn find_optional(&mut self, id: UserId) -> Result<Option<User>, DatabaseError> {
        find_optional(&mut **self.tx, id).await
    }
}

async fn find_optional<'c, T>(executor: T, id: UserId) -> Result<Option<User>, DatabaseError>
where
    T: Executor<'c, Database = Sqlite>,
{
    let result: Option<User> = sqlx::query_as("select id from users where id = $1")
        .bind(id)
        .fetch_optional(executor)
        .await?;
    Ok(result)
}
