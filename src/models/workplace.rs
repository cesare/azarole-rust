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
        let workplaces: Vec<Workplace> = sqlx::query_as("select id, user_id, name from workplaces where user_id = $1")
            .bind(self.user.id)
            .fetch_all(&self.context.database.pool)
            .await?;
        Ok(workplaces)
    }
}
