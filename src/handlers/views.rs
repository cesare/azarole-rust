use chrono::{DateTime, Utc};
use serde::Serialize;

use crate::models::{
    attendance_record,
    AttendanceRecord, AttendanceRecordId, Workplace, WorkplaceId,
};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(in crate::handlers) struct AttendanceRecordView<'a> {
    id: &'a AttendanceRecordId,
    event: &'a attendance_record::Event,
    recoreded_at: &'a DateTime<Utc>,
}

impl<'a> AttendanceRecordView<'a> {
    pub(in crate::handlers) fn new(attendance_record: &'a AttendanceRecord) -> Self {
        Self {
            id: &attendance_record.id,
            event: &attendance_record.event,
            recoreded_at: &attendance_record.recorded_at,
        }
    }
}


#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(in crate::handlers) struct WorkplaceView<'a> {
    id: &'a WorkplaceId,
    name: &'a String,
}

impl<'a> WorkplaceView<'a> {
    pub(in crate::handlers) fn new(workplace: &'a Workplace) -> Self {
        Self {
            id: &workplace.id,
            name: &workplace.name,
        }
    }
}

