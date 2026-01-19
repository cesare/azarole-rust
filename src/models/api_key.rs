use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

use super::{IdType, Timestamp, UserId};

#[derive(Clone, Copy, Deserialize, Serialize, sqlx::Type)]
#[sqlx(transparent)]
#[repr(transparent)]
pub struct ApiKeyId(IdType);

#[derive(Clone, Deserialize, FromRow, Serialize)]
#[allow(dead_code)]
pub struct ApiKey {
    pub id: ApiKeyId,
    pub user_id: UserId,
    pub name: String,
    pub digest: String,
    pub created_at: Timestamp,
}
