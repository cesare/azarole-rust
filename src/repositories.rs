use async_trait::async_trait;
use sqlx::{Pool, Sqlite, Transaction};

use crate::{
    errors::DatabaseError,
    models::{
        ApiKey, ApiKeyId, AttendanceRecord, AttendanceRecordId, Timestamp, User, UserId, Workplace,
        WorkplaceId, attendance_record::Event,
    },
    repositories::{
        api_key::RdbApiKeyRepository,
        attendance_record::RdbAttendanceRecordRepository,
        user::{RdbUserRepository, TxUserRepository},
        workplace::RdbWorkplaceRepository,
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

#[async_trait]
pub trait Repository<'r> {
    type ApiKeyRepository: ApiKeyRepository;
    type AttendanceRecordRepository: AttendanceRecordRepository;
    type UserRepository: UserRepository;
    type WorkplaceRepository: WorkplaceRepository;
    type TransactionalRepository: TransactionalRepository<'r>;

    fn api_key(&'r self) -> Self::ApiKeyRepository;
    fn attendance_record(&'r self) -> Self::AttendanceRecordRepository;
    fn user(&'r self) -> Self::UserRepository;
    fn workplace(&'r self) -> Self::WorkplaceRepository;

    async fn begin(&'r self) -> Result<Self::TransactionalRepository, DatabaseError>;
}

#[derive(Clone)]
pub struct RdbRepository {
    pool: Pool<Sqlite>,
}

impl RdbRepository {
    pub fn new(pool: Pool<Sqlite>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl<'r> Repository<'r> for RdbRepository {
    type ApiKeyRepository = RdbApiKeyRepository<'r>;
    type AttendanceRecordRepository = RdbAttendanceRecordRepository<'r>;
    type UserRepository = RdbUserRepository<'r>;
    type WorkplaceRepository = RdbWorkplaceRepository<'r>;
    type TransactionalRepository = TxRepository<'r>;

    fn api_key(&'r self) -> Self::ApiKeyRepository {
        RdbApiKeyRepository::new(&self.pool)
    }

    fn attendance_record(&'r self) -> Self::AttendanceRecordRepository {
        RdbAttendanceRecordRepository::new(&self.pool)
    }

    fn user(&'r self) -> Self::UserRepository {
        RdbUserRepository::new(&self.pool)
    }

    fn workplace(&'r self) -> Self::WorkplaceRepository {
        RdbWorkplaceRepository::new(&self.pool)
    }

    async fn begin(&'r self) -> Result<Self::TransactionalRepository, DatabaseError> {
        TxRepository::create(&self.pool).await
    }
}

#[async_trait]
pub trait TransactionalUserRepository {
    async fn find_optional(&mut self, id: UserId) -> Result<Option<User>, DatabaseError>;
}

#[async_trait]
pub trait TransactionalRepository<'r> {
    type UserRepository: TransactionalUserRepository;

    fn user(&'r mut self) -> Self::UserRepository;

    async fn rollback(self) -> Result<(), DatabaseError>;
    async fn commit(self) -> Result<(), DatabaseError>;
}

pub struct TxRepository<'r> {
    tx: Transaction<'r, Sqlite>,
}

impl<'r> TxRepository<'r> {
    async fn create(pool: &Pool<Sqlite>) -> Result<Self, DatabaseError> {
        let tx = pool.begin().await?;
        let repository = TxRepository { tx };
        Ok(repository)
    }
}

#[async_trait]
impl<'r> TransactionalRepository<'r> for TxRepository<'r> {
    type UserRepository = TxUserRepository<'r>;

    fn user(&'r mut self) -> Self::UserRepository {
        TxUserRepository::new(&mut self.tx)
    }

    async fn rollback(self) -> Result<(), DatabaseError> {
        let _ = self.tx.rollback().await?;
        Ok(())
    }

    async fn commit(self) -> Result<(), DatabaseError> {
        let _ = self.tx.commit().await?;
        Ok(())
    }
}
