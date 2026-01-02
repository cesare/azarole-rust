use actix_web::{
    HttpResponse,
    web::{Data, Form, Path, Query, ReqData, ServiceConfig, delete, get, post},
};
use chrono::{DateTime, Local};
use serde::Deserialize;
use serde_json::json;
use validator::Validate;

use super::views::{AttendanceRecordView, WorkplaceView};
use crate::{
    context::ApplicationContext,
    errors::PerRequestError,
    models::{
        AttendanceRecordId, AttendanceRecordResources, User, WorkplaceId, WorkplaceResources,
        attendance_record,
    },
};

mod listing;
use listing::{AttendancesForMonth, TargetMonth};

pub(super) fn routes(config: &mut ServiceConfig) {
    config
        .route("", get().to(index))
        .route("", post().to(create))
        .route("/{id}", delete().to(destroy));
}

#[derive(Deserialize)]
struct PathInfo {
    workplace_id: WorkplaceId,
}

#[derive(Deserialize, Validate)]
struct IndexParameters {
    #[validate(range(min = 0))]
    year: Option<i32>,
    #[validate(range(min = 1, max = 12))]
    month: Option<u32>,
}

async fn index(
    context: Data<ApplicationContext>,
    current_user: ReqData<User>,
    path: Path<PathInfo>,
    params: Query<IndexParameters>,
) -> Result<HttpResponse, PerRequestError> {
    let workplace = WorkplaceResources::new(&context, &current_user)
        .find(path.workplace_id)
        .await?;

    let target_month = TargetMonth::new_with_default_timezone(params.year, params.month);
    let finder = AttendancesForMonth::new(&context, &workplace, &target_month);
    let attendance_records = finder.execute().await?;

    let response_json = json!({
        "year": &target_month.year,
        "month": &target_month.month,
        "workplace": WorkplaceView::new(&workplace),
        "attendanceRecords": attendance_records.iter().map(AttendanceRecordView::new).collect::<Vec<AttendanceRecordView>>(),
    });
    let response = HttpResponse::Ok().json(response_json);
    Ok(response)
}

#[derive(Deserialize)]
struct CreationParameters {
    event: attendance_record::Event,
    datetime: DateTime<Local>,
}

async fn create(
    context: Data<ApplicationContext>,
    current_user: ReqData<User>,
    path: Path<PathInfo>,
    form: Form<CreationParameters>,
) -> Result<HttpResponse, PerRequestError> {
    let workplace = WorkplaceResources::new(&context, &current_user)
        .find(path.workplace_id)
        .await?;

    let resources = AttendanceRecordResources::new(&context, &workplace);
    let attendance_record = resources
        .create(&form.event, &form.datetime.to_utc())
        .await?;

    let response_json = json!({
        "attendanceRecord": AttendanceRecordView::new(&attendance_record),
    });
    let response = HttpResponse::Ok().json(response_json);
    Ok(response)
}

#[derive(Deserialize)]
struct DestroyPath {
    workplace_id: WorkplaceId,
    id: AttendanceRecordId,
}

async fn destroy(
    context: Data<ApplicationContext>,
    current_user: ReqData<User>,
    path: Path<DestroyPath>,
) -> Result<HttpResponse, PerRequestError> {
    let workplace = WorkplaceResources::new(&context, &current_user)
        .find(path.workplace_id)
        .await?;

    let resources = AttendanceRecordResources::new(&context, &workplace);
    resources.destroy(path.id).await?;

    let response = HttpResponse::Ok().finish();
    Ok(response)
}
