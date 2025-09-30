use std::sync::Arc;

use chrono::Utc;

use crate::{
    context::ApplicationContext,
    errors::DatabaseError,
    models::{
        AttendanceRecord,
        attendance_record::Event,
        User,
        WorkplaceId,
    }
};

pub struct AttendanceRegistration {
    context: Arc<ApplicationContext>,
}

impl AttendanceRegistration {
    pub fn new(context: Arc<ApplicationContext>) -> Self {
        Self { context }
    }

    pub async fn execute(&self, user: &User, workplace_id: WorkplaceId, event: Event) -> Result<AttendanceRecord, DatabaseError> {
        self.ensure_workplace(user, workplace_id).await?;
        self.create_attendance(workplace_id, event).await
    }

    async fn ensure_workplace(&self, user: &User, workplace_id: WorkplaceId) -> Result<(), DatabaseError> {
        let result = sqlx::query_as::<sqlx::Sqlite, (u32,)>("select 1 from workplaces where user_id = $1 and id = $2")
            .bind(user.id)
            .bind(workplace_id)
            .fetch_one(&self.context.database.pool)
            .await;
        match result {
            Ok(_) => Ok(()),
            Err(error) => Err(error.into()),
        }
    }

    async fn create_attendance(&self, workplace_id: WorkplaceId, event: Event) -> Result<AttendanceRecord, DatabaseError> {
        let now = Utc::now();
        let statement = "insert into attendance_records (workplace_id, event, recorded_at, created_at) values($1, $2, $3, $4) returning id, workplace_id, event, recorded_at";
        let result = sqlx::query_as::<sqlx::Sqlite, AttendanceRecord>(statement)
            .bind(workplace_id)
            .bind(event)
            .bind(now)
            .bind(now)
            .fetch_one(&self.context.database.pool)
            .await;
        match result {
            Ok(attendance_record) => Ok(attendance_record),
            Err(error) => Err(error.into()),
        }
    }
}
