pub mod api_key;
pub mod attendance_record;
pub mod user;
pub mod workplace;

pub use api_key::{ApiKey, ApiKeyId, TokenDigester, TokenGenerator};
pub use attendance_record::{AttendanceRecord, AttendanceRecordId};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
pub use user::{User, UserId};
pub use workplace::{Workplace, WorkplaceId};

type IdType = u32;

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize, sqlx::Type)]
#[sqlx(transparent)]
#[repr(transparent)]
pub struct Timestamp(DateTime<Utc>);

impl From<DateTime<Utc>> for Timestamp {
    fn from(value: DateTime<Utc>) -> Self {
        Self(value)
    }
}
