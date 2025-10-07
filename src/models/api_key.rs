use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

use crate::{
    context::ApplicationContext,
    errors::DatabaseError,
    models::user::{User, UserId}
};

#[derive(Clone, Copy, Deserialize, Serialize, sqlx::Type)]
#[sqlx(transparent)]
#[repr(transparent)]
pub struct ApiKeyId(u32);

#[derive(Clone, Deserialize, FromRow, Serialize)]
#[allow(dead_code)]
pub struct ApiKey {
    pub id: ApiKeyId,
    pub user_id: UserId,
    pub name: String,
    pub digest: String,
}

pub struct ApiKeyResources<'a> {
    context: &'a ApplicationContext,
    user: &'a User,
}

impl<'a> ApiKeyResources<'a> {
    pub fn new(context: &'a ApplicationContext, user: &'a User) -> Self {
        Self { context, user }
    }

    pub async fn list(&self) -> Result<Vec<ApiKey>, DatabaseError> {
        let api_keys: Vec<ApiKey> = sqlx::query_as("select id, name from api_keys where user_id = $1 order by created_at desc")
            .bind(self.user.id)
            .fetch_all(&self.context.database.pool)
            .await?;
        Ok(api_keys)
    }

    pub async fn destroy(&self, id: &ApiKeyId) -> Result<(), DatabaseError> {
        sqlx::query("delete from api_keys where id = $1")
            .bind(id)
            .execute(&self.context.database.pool)
            .await?;
        Ok(())
    }
}
