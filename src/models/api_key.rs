use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

use crate::models::user::UserId;

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
    pub created_at: DateTime<Utc>,
}
