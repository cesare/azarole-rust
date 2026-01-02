use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

use crate::models::UserId;

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
