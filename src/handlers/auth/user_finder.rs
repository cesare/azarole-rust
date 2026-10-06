use crate::{
    AppState,
    errors::DatabaseError,
    models::User,
    repositories::{Repository, TransactionalRepository},
};

pub(super) struct UserFinder<'a> {
    app_state: &'a AppState,
}

impl<'a> UserFinder<'a> {
    pub(super) fn new(app_state: &'a AppState) -> Self {
        Self { app_state }
    }

    pub(super) async fn execute(self, identifier: &str) -> Result<User, DatabaseError> {
        let mut txr = self.app_state.repository.begin().await?;

        if let Some(user) = txr.find_optional_user_with_google_uid(identifier).await? {
            txr.rollback().await?;
            return Ok(user);
        }

        let user = txr.create_user().await?;
        txr.create_google_uid(&user, identifier).await?;
        txr.commit().await?;

        Ok(user)
    }
}
