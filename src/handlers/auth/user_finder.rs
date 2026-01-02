use chrono::Utc;
use sqlx::SqliteConnection;

use crate::{context::ApplicationContext, errors::DatabaseError, models::User};

pub(super) struct UserFinder<'a> {
    context: &'a ApplicationContext,
    identifier: &'a str,
}

impl<'a> UserFinder<'a> {
    pub(super) fn new(context: &'a ApplicationContext, identifier: &'a str) -> Self {
        Self {
            context,
            identifier,
        }
    }

    pub(super) async fn execute(self) -> Result<User, DatabaseError> {
        let mut tx = self
            .context
            .database
            .pool
            .begin()
            .await
            .inspect_err(|e| log::error!("Failed to begin transaction: {:?}", e))?;

        if let Some(user) = self.find(&mut tx).await? {
            return Ok(user);
        }

        let user = self.create(&mut tx).await?;
        tx.commit()
            .await
            .inspect_err(|e| log::error!("Failed to commit transaction: {:?}", e))?;

        Ok(user)
    }

    async fn find(&self, connection: &mut SqliteConnection) -> Result<Option<User>, DatabaseError> {
        let user: Option<User> =
            sqlx::query_as("select user_id as id from google_authenticated_users where uid = $1")
                .bind(self.identifier)
                .fetch_optional(connection)
                .await
                .inspect_err(|e| log::error!("Failed to find user: {:?}", e))?;
        Ok(user)
    }

    async fn create(&self, connection: &mut SqliteConnection) -> Result<User, DatabaseError> {
        let now = Utc::now();

        let user: User = sqlx::query_as("insert into users (created_at) values ($1) returning id")
            .bind(now)
            .fetch_one(&mut *connection)
            .await
            .inspect_err(|e| log::error!("Failed to create user: {:?}", e))?;

        sqlx::query(
            "insert into google_authenticated_users (user_id, uid, created_at) values ($1, $2, $3)",
        )
        .bind(user.id)
        .bind(self.identifier)
        .bind(now)
        .execute(connection)
        .await
        .inspect_err(|e| log::error!("Failed to insert google_authenticated_users: {:?}", e))?;

        Ok(user)
    }
}
