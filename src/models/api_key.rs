use sqlx::prelude::FromRow;

use crate::models::user::UserId;

#[derive(FromRow)]
#[allow(dead_code)]
pub struct ApiKey {
    pub id: u32,
    pub user_id: UserId,
    pub name: String,
    pub digest: String,
}
