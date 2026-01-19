use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

use super::IdType;

#[derive(Clone, Copy, Deserialize, Serialize, sqlx::Type)]
#[sqlx(transparent)]
#[repr(transparent)]
pub struct UserId(IdType);

#[derive(Clone, Deserialize, FromRow, Serialize)]
pub struct User {
    pub id: UserId,
}

impl User {
    pub fn new(id: UserId) -> Self {
        Self { id }
    }
}

impl From<IdType> for UserId {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
