use std::sync::Arc;

use chrono::Utc;

use crate::{
    context::ApplicationContext,
    errors::DatabaseError,
    models::{AttendanceRecord, User, WorkplaceId, attendance_record::Event},
    repositories::RepositoryFactory,
};

pub(super) struct AttendanceRegistration {
    context: Arc<ApplicationContext>,
}

impl AttendanceRegistration {
    pub(super) fn new(context: Arc<ApplicationContext>) -> Self {
        Self { context }
    }

    pub(super) async fn execute(
        &self,
        user: &User,
        workplace_id: WorkplaceId,
        event: Event,
    ) -> Result<AttendanceRecord, DatabaseError> {
        let workplace = self
            .context
            .repositories
            .workplace()
            .find(user, workplace_id)
            .await?;
        let repository = self.context.repositories.attendance_record();
        repository.create(&workplace, &event, &Utc::now()).await
    }
}
