use sqlx::prelude::FromRow;

#[derive(FromRow)]
pub struct ApiKey {
    pub id: u32,
    pub user_id: u32,
    pub name: String,
    pub digest: String,
}
