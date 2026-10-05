use async_trait::async_trait;
use chrono::Utc;
use sqlx::{Executor, Pool, Sqlite};

use crate::{
    errors::DatabaseError,
    models::{
        AttendanceRecord, AttendanceRecordId, Timestamp, Workplace, attendance_record::Event,
    },
    repositories::AttendanceRecordRepository,
};

pub struct RdbAttendanceRecordRepository<'r> {
    pool: &'r Pool<Sqlite>,
}

impl<'r> RdbAttendanceRecordRepository<'r> {
    pub fn new(pool: &'r Pool<Sqlite>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl<'r> AttendanceRecordRepository for RdbAttendanceRecordRepository<'r> {
    async fn create(
        &self,
        workplace: &Workplace,
        event: &Event,
        datetime: &Timestamp,
    ) -> Result<AttendanceRecord, DatabaseError> {
        create(self.pool, workplace, event, datetime).await
    }

    async fn destroy(
        &self,
        workplace: &Workplace,
        id: AttendanceRecordId,
    ) -> Result<(), DatabaseError> {
        destroy(self.pool, workplace, id).await
    }

    async fn list(
        &self,
        workplace: &Workplace,
        start_time: &Timestamp,
        end_time: &Timestamp,
    ) -> Result<Vec<AttendanceRecord>, DatabaseError> {
        list(self.pool, workplace, start_time, end_time).await
    }
}

async fn create<'c, T>(
    executor: T,
    workplace: &Workplace,
    event: &Event,
    datetime: &Timestamp,
) -> Result<AttendanceRecord, DatabaseError>
where
    T: Executor<'c, Database = Sqlite>,
{
    let statement = "insert into attendance_records (workplace_id, event, recorded_at, created_at) values ($1, $2, $3, $4) returning id, workplace_id, event, recorded_at";
    let now = Utc::now();
    let attendance_record = sqlx::query_as(statement)
        .bind(workplace.id)
        .bind(event)
        .bind(datetime)
        .bind(now)
        .fetch_one(executor)
        .await
        .inspect_err(|e| log::error!("Failed to create attendance_record: {:?}", e))?;

    Ok(attendance_record)
}

async fn destroy<'c, T>(
    executor: T,
    workplace: &Workplace,
    id: AttendanceRecordId,
) -> Result<(), DatabaseError>
where
    T: Executor<'c, Database = Sqlite>,
{
    let statement = "delete from attendance_records where id = $1 and workplace_id = $2";
    sqlx::query(statement)
        .bind(id)
        .bind(workplace.id)
        .execute(executor)
        .await
        .inspect_err(|e| log::error!("Failed to delete attendance_record: {:?}", e))?;

    Ok(())
}

async fn list<'c, T>(
    executor: T,
    workplace: &Workplace,
    start_time: &Timestamp,
    end_time: &Timestamp,
) -> Result<Vec<AttendanceRecord>, DatabaseError>
where
    T: Executor<'c, Database = Sqlite>,
{
    let statement = "select id, workplace_id, event, recorded_at from attendance_records where workplace_id = $1 and recorded_at >= $2 and recorded_at < $3 order by recorded_at";
    let attendance_records: Vec<AttendanceRecord> = sqlx::query_as(statement)
        .bind(workplace.id)
        .bind(start_time)
        .bind(end_time)
        .fetch_all(executor)
        .await?;
    Ok(attendance_records)
}
