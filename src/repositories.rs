use async_trait::async_trait;
use chrono::Utc;
use sqlx::{Pool, Sqlite, Transaction};

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

#[async_trait]
pub trait Repository<'r> {
    type ApiKeyRepository: ApiKeyRepository;
    type AttendanceRecordRepository: AttendanceRecordRepository;
    type UserRepository: UserRepository;
    type WorkplaceRepository: WorkplaceRepository;
    type TransactionalRepository: TransactionalRepository;

    fn api_key(&'r self) -> Self::ApiKeyRepository;
    fn attendance_record(&'r self) -> Self::AttendanceRecordRepository;
    fn user(&'r self) -> Self::UserRepository;
    fn workplace(&'r self) -> Self::WorkplaceRepository;

    async fn begin(&self) -> Result<Self::TransactionalRepository, DatabaseError>;
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

    async fn begin(&self) -> Result<Self::TransactionalRepository, DatabaseError> {
        TxRepository::create(&self.pool).await
    }
}

#[async_trait]
pub trait TransactionalRepository {
    async fn find_optional_user_with_google_uid(
        &mut self,
        uid: &str,
    ) -> Result<Option<User>, DatabaseError>;
    async fn create_user(&mut self) -> Result<User, DatabaseError>;
    async fn create_google_uid(&mut self, user: &User, uid: &str) -> Result<(), DatabaseError>;

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
impl<'r> TransactionalRepository for TxRepository<'r> {
    async fn find_optional_user_with_google_uid(
        &mut self,
        uid: &str,
    ) -> Result<Option<User>, DatabaseError> {
        let statement = "select user_id as id from google_authenticated_users where uid = $1";
        let user: Option<User> = sqlx::query_as(statement)
            .bind(uid)
            .fetch_optional(&mut *self.tx)
            .await
            .inspect_err(|e| log::error!("Failed to find user: {:?}", e))?;
        Ok(user)
    }

    async fn create_user(&mut self) -> Result<User, DatabaseError> {
        let now = Utc::now();

        let statement = "insert into users (created_at) values ($1) returning id";
        let user: User = sqlx::query_as(statement)
            .bind(now)
            .fetch_one(&mut *self.tx)
            .await
            .inspect_err(|e| log::error!("Failed to create user: {:?}", e))?;
        Ok(user)
    }

    async fn create_google_uid(&mut self, user: &User, uid: &str) -> Result<(), DatabaseError> {
        let now = Utc::now();

        let statement =
            "insert into google_authenticated_users (user_id, uid, created_at) values ($1, $2, $3)";
        sqlx::query(statement)
            .bind(user.id)
            .bind(uid)
            .bind(now)
            .execute(&mut *self.tx)
            .await
            .inspect_err(|e| log::error!("Failed to insert google_authenticated_users: {:?}", e))?;
        Ok(())
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
