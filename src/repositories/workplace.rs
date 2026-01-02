use std::marker::PhantomData;

use async_trait::async_trait;
use chrono::Utc;
use sqlx::{Executor, Sqlite};

use crate::{
    errors::DatabaseError,
    models::{User, Workplace, WorkplaceId},
    repositories::WorkplaceRepository,
};

pub struct RdbWorkplaceRepository<'a, T: Executor<'a>> {
    executor: T,
    _marker: PhantomData<&'a T>,
}

impl<'a, T> RdbWorkplaceRepository<'a, T>
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
impl<'a, T> WorkplaceRepository for RdbWorkplaceRepository<'a, T>
where
    T: Executor<'a, Database = Sqlite> + Copy + Sync,
{
    async fn list(&self, user: &User) -> Result<Vec<Workplace>, DatabaseError> {
        let workplaces: Vec<Workplace> = sqlx::query_as(
            "select id, user_id, name from workplaces where user_id = $1 order by id",
        )
        .bind(user.id)
        .fetch_all(self.executor)
        .await
        .inspect_err(|e| log::error!("Failed to query workplaces: {:?}", e))?;

        Ok(workplaces)
    }

    async fn create(&self, user: &User, name: &str) -> Result<Workplace, DatabaseError> {
        let statement = "insert into workplaces (user_id, name, created_at, updated_at) values ($1, $2, $3, $4) returning id, user_id, name";
        let now = Utc::now();

        let workplace: Workplace = sqlx::query_as(statement)
            .bind(user.id)
            .bind(name)
            .bind(now)
            .bind(now)
            .fetch_one(self.executor)
            .await
            .inspect_err(|e| log::error!("Failed to create workplace: {:?}", e))?;

        Ok(workplace)
    }

    async fn find(&self, user: &User, id: WorkplaceId) -> Result<Workplace, DatabaseError> {
        let statement = "select id, user_id, name from workplaces where user_id = $1 and id = $2";
        let workplace: Workplace = sqlx::query_as(statement)
            .bind(user.id)
            .bind(id)
            .fetch_one(self.executor)
            .await
            .inspect_err(|e| log::error!("Failed to find workplace: {:?}", e))?;

        Ok(workplace)
    }
}
