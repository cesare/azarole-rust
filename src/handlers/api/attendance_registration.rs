use std::sync::Arc;

use chrono::Utc;

use crate::{
    context::AppState,
    errors::DatabaseError,
    models::{AttendanceRecord, User, WorkplaceId, attendance_record::Event},
    repositories::RepositoryFactory,
};

pub(super) struct AttendanceRegistration {
    app_state: Arc<AppState>,
}

impl AttendanceRegistration {
    pub(super) fn new(app_state: Arc<AppState>) -> Self {
        Self { app_state }
    }

    pub(super) async fn execute(
        &self,
        user: &User,
        workplace_id: WorkplaceId,
        event: Event,
    ) -> Result<AttendanceRecord, DatabaseError> {
        let workplace = self
            .app_state
            .repositories
            .workplace()
            .find(user, workplace_id)
            .await?;
        let repository = self.app_state.repositories.attendance_record();
        repository
            .create(&workplace, &event, &Utc::now().into())
            .await
    }
}
