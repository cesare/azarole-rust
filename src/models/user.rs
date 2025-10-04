use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Clone, Copy, Deserialize, Serialize, sqlx::Type)]
#[sqlx(transparent)]
#[repr(transparent)]
pub struct UserId(u32);

#[derive(Clone, Deserialize, FromRow, Serialize)]
pub struct User {
    pub id: UserId,
}

impl User {
    pub fn new(id: UserId) -> Self {
        Self { id }
    }
}
