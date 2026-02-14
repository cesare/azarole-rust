use std::sync::Arc;

use actix_web::web::{Data, Path, ReqData, ServiceConfig, post};
use actix_web::{HttpResponse, Result};
use serde_json::json;

use crate::{
    context::AppState,
    errors::PerRequestError,
    models::{User, WorkplaceId, attendance_record::Event},
};

mod attendance_registration;
use attendance_registration::AttendanceRegistration;

pub(super) fn routes(config: &mut ServiceConfig) {
    config
        .route("/workplaces/{workplace_id}/clock_ins", post().to(clock_in))
        .route(
            "/workplaces/{workplace_id}/clock_outs",
            post().to(clock_out),
        );
}

async fn clock_in(
    context: Data<AppState>,
    current_user: ReqData<User>,
    path: Path<WorkplaceId>,
) -> Result<HttpResponse, PerRequestError> {
    create_clock(context, current_user, path, Event::ClockIn).await
}

async fn clock_out(
    context: Data<AppState>,
    current_user: ReqData<User>,
    path: Path<WorkplaceId>,
) -> Result<HttpResponse, PerRequestError> {
    create_clock(context, current_user, path, Event::ClockOut).await
}

async fn create_clock(
    context: Data<AppState>,
    current_user: ReqData<User>,
    path: Path<WorkplaceId>,
    event: Event,
) -> Result<HttpResponse, PerRequestError> {
    let workplace_id = path.into_inner();

    let registration = AttendanceRegistration::new(Arc::clone(&context.into_inner()));
    let attendance_record = registration
        .execute(&current_user, workplace_id, event)
        .await?;

    let response_json = json!({
        "attendanceRecord": {
            "id": attendance_record.id,
            "workplaceId": attendance_record.workplace_id,
            "event": attendance_record.event,
            "recordedAt": attendance_record.recorded_at,
        },
    });
    let response = HttpResponse::Created().json(response_json);
    Ok(response)
}
