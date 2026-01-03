use std::marker::PhantomData;

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{Executor, Sqlite};

use crate::{
    errors::DatabaseError,
    models::{AttendanceRecord, AttendanceRecordId, Workplace, attendance_record::Event},
    repositories::AttendanceRecordRepository,
};

pub struct RdbAttendanceRecordRepository<'a, T: Executor<'a>> {
    executor: T,
    _marker: PhantomData<&'a T>,
}

impl<'a, T> RdbAttendanceRecordRepository<'a, T>
where
    T: Executor<'a, Database = Sqlite> + Copy + Sync,
{
    pub fn new(executor: T) -> Self {
        Self {
            executor,
            _marker: PhantomData,
        }
    }
}

#[async_trait]
impl<'a, T> AttendanceRecordRepository for RdbAttendanceRecordRepository<'a, T>
where
    T: Executor<'a, Database = Sqlite> + Copy + Sync,
{
    async fn create(
        &self,
        workplace: &Workplace,
        event: &Event,
        datetime: &DateTime<Utc>,
    ) -> Result<AttendanceRecord, DatabaseError> {
        let statement = "insert into attendance_records (workplace_id, event, recorded_at, created_at) values ($1, $2, $3, $4) returning id, workplace_id, event, recorded_at";
        let now = Utc::now();
        let attendance_record = sqlx::query_as(statement)
            .bind(workplace.id)
            .bind(event)
            .bind(datetime)
            .bind(now)
            .fetch_one(self.executor)
            .await
            .inspect_err(|e| log::error!("Failed to create attendance_record: {:?}", e))?;

        Ok(attendance_record)
    }

    async fn destroy(
        &self,
        workplace: &Workplace,
        id: AttendanceRecordId,
    ) -> Result<(), DatabaseError> {
        let statement = "delete from attendance_records where id = $1 and workplace_id = $2";
        sqlx::query(statement)
            .bind(id)
            .bind(workplace.id)
            .execute(self.executor)
            .await
            .inspect_err(|e| log::error!("Failed to delete attendance_record: {:?}", e))?;

        Ok(())
    }

    async fn list(
        &self,
        workplace: &Workplace,
        start_time: &DateTime<Utc>,
        end_time: &DateTime<Utc>,
    ) -> Result<Vec<AttendanceRecord>, DatabaseError> {
        let statement = "select id, workplace_id, event, recorded_at from attendance_records where workplace_id = $1 and recorded_at >= $2 and recorded_at < $3 order by recorded_at";
        let attendance_records: Vec<AttendanceRecord> = sqlx::query_as(statement)
            .bind(workplace.id)
            .bind(start_time)
            .bind(end_time)
            .fetch_all(self.executor)
            .await?;
        Ok(attendance_records)
    }
}
