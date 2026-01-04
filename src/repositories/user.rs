use std::marker::PhantomData;

use async_trait::async_trait;
use sqlx::{Executor, Sqlite};

use crate::{
    errors::DatabaseError,
    models::{User, UserId},
};

use super::UserRepository;

pub struct RdbUserRepository<'a, T: Executor<'a>> {
    executor: T,
    _marker: PhantomData<&'a T>,
}

impl<'a, T> RdbUserRepository<'a, T>
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
impl<'a, T> UserRepository for RdbUserRepository<'a, T>
where
    T: Executor<'a, Database = Sqlite> + Copy + Sync,
{
    async fn find_optional(&self, id: UserId) -> Result<Option<User>, DatabaseError> {
        let result: Option<User> = sqlx::query_as("select id from users where id = $1")
            .bind(id)
            .fetch_optional(self.executor)
            .await?;
        Ok(result)
    }
}
