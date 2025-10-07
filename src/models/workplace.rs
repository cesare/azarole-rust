use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

use crate::{context::ApplicationContext, errors::DatabaseError};

use super::{User, UserId};

#[derive(Clone, Copy, Deserialize, Serialize, sqlx::Type)]
#[sqlx(transparent)]
#[repr(transparent)]
pub struct WorkplaceId(u32);

#[derive(Clone, Deserialize, FromRow, Serialize)]
pub struct Workplace {
    pub id: WorkplaceId,
    pub user_id: UserId,
    pub name: String,
}

pub struct WorkplaceResources<'a> {
    context: &'a ApplicationContext,
    user: &'a User,
}

impl<'a> WorkplaceResources<'a> {
    pub fn new(context: &'a ApplicationContext, user: &'a User) -> Self {
        Self { context, user }
    }

    pub async fn list(&self) -> Result<Vec<Workplace>, DatabaseError> {
        let workplaces: Vec<Workplace> = sqlx::query_as("select id, user_id, name from workplaces where user_id = $1 order by id")
            .bind(self.user.id)
            .fetch_all(&self.context.database.pool)
            .await?;
        Ok(workplaces)
    }

    pub async fn create(&self, name: &str) -> Result<Workplace, DatabaseError> {
        let statement = "insert into workplaces (user_id, name, created_at updated_at) values ($1, $2, $3, $4) returning id, user_id, name";
        let now = Utc::now();

        let workplace: Workplace = sqlx::query_as(statement)
            .bind(self.user.id)
            .bind(name)
            .bind(now)
            .bind(now)
            .fetch_one(&self.context.database.pool)
            .await?;
        Ok(workplace)
    }

    pub async fn find(&self, id: WorkplaceId) -> Result<Workplace, DatabaseError> {
        let statement = "select id, user_id, name from workplaces where user_id = $1 and id = $2";
        let workplace: Workplace = sqlx::query_as(statement)
            .bind(self.user.id)
            .bind(id)
            .fetch_one(&self.context.database.pool)
            .await?;
        Ok(workplace)
    }
}
