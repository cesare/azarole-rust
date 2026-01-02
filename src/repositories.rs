use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{Pool, Sqlite};

use crate::{
    errors::DatabaseError,
    models::{
        ApiKey, ApiKeyId, AttendanceRecord, AttendanceRecordId, User, Workplace,
        attendance_record::Event,
    },
    repositories::{
        api_key::RdbApiKeyRepository, attendance_record::RdbAttendanceRecordRepository,
    },
};

mod api_key;
mod attendance_record;

#[async_trait]
pub trait ApiKeyRepository {
    async fn list(&self, user: &User) -> Result<Vec<ApiKey>, DatabaseError>;
    async fn destroy(&self, user: &User, id: &ApiKeyId) -> Result<(), DatabaseError>;
}

#[async_trait]
pub trait AttendanceRecordRepository {
    async fn create(
        &self,
        workplace: &Workplace,
        event: &Event,
        datetime: &DateTime<Utc>,
    ) -> Result<AttendanceRecord, DatabaseError>;
    async fn destroy(
        &self,
        workplace: &Workplace,
        id: AttendanceRecordId,
    ) -> Result<(), DatabaseError>;
}

pub trait RepositoryFactory {
    fn api_key(&self) -> Box<dyn ApiKeyRepository + '_>;
    fn attendance_record(&self) -> Box<dyn AttendanceRecordRepository + '_>;
}

#[derive(Clone)]
pub struct RdbRepositories {
    pool: Pool<Sqlite>,
}

impl RdbRepositories {
    pub fn new(pool: Pool<Sqlite>) -> Self {
        Self { pool }
    }
}

impl RepositoryFactory for RdbRepositories {
    fn api_key(&self) -> Box<dyn ApiKeyRepository + '_> {
        Box::new(RdbApiKeyRepository::new(&self.pool))
    }

    fn attendance_record(&self) -> Box<dyn AttendanceRecordRepository + '_> {
        Box::new(RdbAttendanceRecordRepository::new(&self.pool))
    }
}
