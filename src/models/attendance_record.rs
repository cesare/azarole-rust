use chrono::{DateTime, Utc};
use sqlx::FromRow;
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize, sqlx::Type)]
#[sqlx(rename_all = "kebab-case")]
#[serde(rename_all = "kebab-case")]
pub enum Event {
    ClockIn,
    ClockOut,
}

#[derive(Clone, Deserialize, FromRow, Serialize)]
pub struct AttendanceRecord {
    pub id: u32,
    pub workplace_id: u32,
    pub event: Event,
    pub recorded_at: DateTime<Utc>,
}
