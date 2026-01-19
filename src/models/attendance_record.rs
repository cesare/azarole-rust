use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use super::{IdType, Timestamp, WorkplaceId};

#[derive(Clone, Copy, Deserialize, Serialize, sqlx::Type)]
#[sqlx(transparent)]
#[repr(transparent)]
pub struct AttendanceRecordId(IdType);

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
    pub recorded_at: Timestamp,
}
