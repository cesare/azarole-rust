use serde::Serialize;

use crate::models::{
    ApiKey, ApiKeyId, AttendanceRecord, AttendanceRecordId, Timestamp, User, UserId, Workplace,
    WorkplaceId, attendance_record,
};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(in crate::handlers) struct ApiKeyView<'a> {
    id: &'a ApiKeyId,
    name: &'a String,
    created_at: &'a Timestamp,
}

impl<'a> ApiKeyView<'a> {
    pub(in crate::handlers) fn new(api_key: &'a ApiKey) -> Self {
        Self {
            id: &api_key.id,
            name: &api_key.name,
            created_at: &api_key.created_at,
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(in crate::handlers) struct AttendanceRecordView<'a> {
    id: &'a AttendanceRecordId,
    event: &'a attendance_record::Event,
    recorded_at: &'a Timestamp,
}

impl<'a> AttendanceRecordView<'a> {
    pub(in crate::handlers) fn new(attendance_record: &'a AttendanceRecord) -> Self {
        Self {
            id: &attendance_record.id,
            event: &attendance_record.event,
            recorded_at: &attendance_record.recorded_at,
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

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(in crate::handlers) struct UserView<'a> {
    id: &'a UserId,
}

impl<'a> UserView<'a> {
    pub(in crate::handlers) fn new(user: &'a User) -> Self {
        Self { id: &user.id }
    }
}
