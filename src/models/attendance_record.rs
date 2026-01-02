use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::{
    context::ApplicationContext,
    errors::DatabaseError,
    models::{Workplace, WorkplaceId},
};

#[derive(Clone, Copy, Deserialize, Serialize, sqlx::Type)]
#[sqlx(transparent)]
#[repr(transparent)]
pub struct AttendanceRecordId(u32);

#[derive(Clone, Deserialize, Serialize, sqlx::Type)]
#[sqlx(rename_all = "kebab-case")]
#[serde(rename_all = "kebab-case")]
pub enum Event {
    ClockIn,
    ClockOut,
}

#[derive(Clone, Deserialize, FromRow, Serialize)]
pub struct AttendanceRecord {
    pub id: AttendanceRecordId,
    pub workplace_id: WorkplaceId,
    pub event: Event,
    pub recorded_at: DateTime<Utc>,
}

pub struct AttendanceRecordResources<'a> {
    context: &'a ApplicationContext,
    workplace: &'a Workplace,
}

impl<'a> AttendanceRecordResources<'a> {
    pub fn new(context: &'a ApplicationContext, workplace: &'a Workplace) -> Self {
        Self { context, workplace }
    }

    pub async fn create(
        &self,
        event: &Event,
        datetime: &DateTime<Utc>,
    ) -> Result<AttendanceRecord, DatabaseError> {
        let statement = "insert into attendance_records (workplace_id, event, recorded_at, created_at) values ($1, $2, $3, $4) returning id, workplace_id, event, recorded_at";
        let now = Utc::now();
        let attendance_record = sqlx::query_as(statement)
            .bind(self.workplace.id)
            .bind(event)
            .bind(datetime)
            .bind(now)
            .fetch_one(&self.context.database.pool)
            .await
            .inspect_err(|e| log::error!("Failed to create attendance_record: {:?}", e))?;

        Ok(attendance_record)
    }

    pub async fn destroy(&self, id: AttendanceRecordId) -> Result<(), DatabaseError> {
        let statement = "delete from attendance_records where id = $1 and workplace_id = $2";
        sqlx::query(statement)
            .bind(id)
            .bind(self.workplace.id)
            .execute(&self.context.database.pool)
            .await
            .inspect_err(|e| log::error!("Failed to delete attendance_record: {:?}", e))?;

        Ok(())
    }
}
