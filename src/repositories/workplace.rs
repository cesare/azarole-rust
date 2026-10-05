use async_trait::async_trait;
use chrono::Utc;
use sqlx::{Executor, Pool, Sqlite};

use crate::{
    errors::DatabaseError,
    models::{User, Workplace, WorkplaceId},
    repositories::WorkplaceRepository,
};

pub struct RdbWorkplaceRepository<'r> {
    pool: &'r Pool<Sqlite>,
}

impl<'r> RdbWorkplaceRepository<'r> {
    pub fn new(pool: &'r Pool<Sqlite>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl<'r> WorkplaceRepository for RdbWorkplaceRepository<'r> {
    async fn list(&self, user: &User) -> Result<Vec<Workplace>, DatabaseError> {
        list(self.pool, user).await
    }

    async fn create(&self, user: &User, name: &str) -> Result<Workplace, DatabaseError> {
        create(self.pool, user, name).await
    }

    async fn find(&self, user: &User, id: WorkplaceId) -> Result<Workplace, DatabaseError> {
        find(self.pool, user, id).await
    }
}

async fn list<'c, T>(executor: T, user: &User) -> Result<Vec<Workplace>, DatabaseError>
where
    T: Executor<'c, Database = Sqlite>,
{
    let workplaces: Vec<Workplace> =
        sqlx::query_as("select id, user_id, name from workplaces where user_id = $1 order by id")
            .bind(user.id)
            .fetch_all(executor)
            .await
            .inspect_err(|e| log::error!("Failed to query workplaces: {:?}", e))?;

    Ok(workplaces)
}

async fn create<'c, T>(executor: T, user: &User, name: &str) -> Result<Workplace, DatabaseError>
where
    T: Executor<'c, Database = Sqlite>,
{
    let statement = "insert into workplaces (user_id, name, created_at, updated_at) values ($1, $2, $3, $4) returning id, user_id, name";
    let now = Utc::now();

    let workplace: Workplace = sqlx::query_as(statement)
        .bind(user.id)
        .bind(name)
        .bind(now)
        .bind(now)
        .fetch_one(executor)
        .await
        .inspect_err(|e| log::error!("Failed to create workplace: {:?}", e))?;

    Ok(workplace)
}

async fn find<'c, T>(executor: T, user: &User, id: WorkplaceId) -> Result<Workplace, DatabaseError>
where
    T: Executor<'c, Database = Sqlite>,
{
    let statement = "select id, user_id, name from workplaces where user_id = $1 and id = $2";
    let workplace: Workplace = sqlx::query_as(statement)
        .bind(user.id)
        .bind(id)
        .fetch_one(executor)
        .await
        .inspect_err(|e| log::error!("Failed to find workplace: {:?}", e))?;

    Ok(workplace)
}
