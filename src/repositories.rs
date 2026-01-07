use async_trait::async_trait;
use sqlx::{Pool, Sqlite};

use crate::{
    errors::DatabaseError,
    models::{
        ApiKey, ApiKeyId, AttendanceRecord, AttendanceRecordId, Timestamp, User, UserId, Workplace,
        WorkplaceId, attendance_record::Event,
    },
    repositories::{
        api_key::RdbApiKeyRepository, attendance_record::RdbAttendanceRecordRepository,
        user::RdbUserRepository, workplace::RdbWorkplaceRepository,
    },
};

mod api_key;
mod attendance_record;
mod user;
mod workplace;

#[async_trait]
pub trait ApiKeyRepository {
    async fn list(&self, user: &User) -> Result<Vec<ApiKey>, DatabaseError>;
    async fn find_by_digest(&self, digest: &str) -> Result<Option<ApiKey>, DatabaseError>;
    async fn create(&self, user: &User, name: &str, digest: &str) -> Result<ApiKey, DatabaseError>;
    async fn destroy(&self, user: &User, id: &ApiKeyId) -> Result<(), DatabaseError>;
}

#[async_trait]
pub trait AttendanceRecordRepository {
    async fn create(
        &self,
        workplace: &Workplace,
        event: &Event,
        datetime: &Timestamp,
    ) -> Result<AttendanceRecord, DatabaseError>;
    async fn destroy(
        &self,
        workplace: &Workplace,
        id: AttendanceRecordId,
    ) -> Result<(), DatabaseError>;
    async fn list(
        &self,
        workplace: &Workplace,
        start_time: &Timestamp,
        end_time: &Timestamp,
    ) -> Result<Vec<AttendanceRecord>, DatabaseError>;
}

#[async_trait]
pub trait UserRepository {
    async fn find_optional(&self, id: UserId) -> Result<Option<User>, DatabaseError>;
}

#[async_trait]
pub trait WorkplaceRepository {
    async fn list(&self, user: &User) -> Result<Vec<Workplace>, DatabaseError>;
    async fn create(&self, user: &User, name: &str) -> Result<Workplace, DatabaseError>;
    async fn find(&self, user: &User, id: WorkplaceId) -> Result<Workplace, DatabaseError>;
}

pub trait RepositoryFactory {
    fn api_key(&self) -> Box<dyn ApiKeyRepository + '_>;
    fn attendance_record(&self) -> Box<dyn AttendanceRecordRepository + '_>;
    fn user(&self) -> Box<dyn UserRepository + '_>;
    fn workplace(&self) -> Box<dyn WorkplaceRepository + '_>;
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

    fn user(&self) -> Box<dyn UserRepository + '_> {
        Box::new(RdbUserRepository::new(&self.pool))
    }

    fn workplace(&self) -> Box<dyn WorkplaceRepository + '_> {
        Box::new(RdbWorkplaceRepository::new(&self.pool))
    }
}
