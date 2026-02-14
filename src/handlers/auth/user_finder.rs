use chrono::Utc;
use sqlx::SqliteConnection;

use crate::{context::AppState, errors::DatabaseError, models::User};

pub(super) struct UserFinder<'a> {
    app_state: &'a AppState,
}

impl<'a> UserFinder<'a> {
    pub(super) fn new(app_state: &'a AppState) -> Self {
        Self { app_state }
    }

    pub(super) async fn execute(self, identifier: &str) -> Result<User, DatabaseError> {
        let mut tx = self
            .app_state
            .database
            .pool
            .begin()
            .await
            .inspect_err(|e| log::error!("Failed to begin transaction: {:?}", e))?;

        if let Some(user) = self.find(&mut tx, identifier).await? {
            return Ok(user);
        }

        let user = self.create(&mut tx, identifier).await?;
        tx.commit()
            .await
            .inspect_err(|e| log::error!("Failed to commit transaction: {:?}", e))?;

        Ok(user)
    }

    async fn find(
        &self,
        connection: &mut SqliteConnection,
        identifier: &str,
    ) -> Result<Option<User>, DatabaseError> {
        let user: Option<User> =
            sqlx::query_as("select user_id as id from google_authenticated_users where uid = $1")
                .bind(identifier)
                .fetch_optional(connection)
                .await
                .inspect_err(|e| log::error!("Failed to find user: {:?}", e))?;
        Ok(user)
    }

    async fn create(
        &self,
        connection: &mut SqliteConnection,
        identifier: &str,
    ) -> Result<User, DatabaseError> {
        let now = Utc::now();

        let user: User = sqlx::query_as("insert into users (created_at) values ($1) returning id")
            .bind(now)
            .fetch_one(&mut *connection)
            .await
            .inspect_err(|e| log::error!("Failed to create user: {:?}", e))?;

        let statement =
            "insert into google_authenticated_users (user_id, uid, created_at) values ($1, $2, $3)";
        sqlx::query(statement)
            .bind(user.id)
            .bind(identifier)
            .bind(now)
            .execute(connection)
            .await
            .inspect_err(|e| log::error!("Failed to insert google_authenticated_users: {:?}", e))?;

        Ok(user)
    }
}
