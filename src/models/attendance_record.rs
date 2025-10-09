use chrono::{DateTime, Utc};
use sqlx::FromRow;
use serde::{Deserialize, Serialize};

use crate::{context::ApplicationContext, errors::DatabaseError, models::{Workplace, WorkplaceId}};

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

    pub async fn destroy(&self, id: AttendanceRecordId) -> Result<(), DatabaseError> {
        let statement = "delete from attendance_records where id = $1 and workplace_id = $2";
        sqlx::query(statement)
            .bind(id)
            .bind(self.workplace.id)
            .execute(&self.context.database.pool)
            .await?;
        Ok(())
    }
}
