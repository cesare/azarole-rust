use chrono::{DateTime, Utc};
use sqlx::FromRow;
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, FromRow, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AttendanceRecord {
    pub id: u32,
    pub workplace_id: u32,
    pub event: String,
    pub recorded_at: DateTime<Utc>,
}
