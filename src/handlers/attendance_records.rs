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
    context::AppState,
    errors::PerRequestError,
    models::{AttendanceRecordId, User, WorkplaceId, attendance_record},
    repositories::RepositoryFactory,
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
    app_state: Data<AppState>,
    current_user: ReqData<User>,
    path: Path<PathInfo>,
    params: Query<IndexParameters>,
) -> Result<HttpResponse, PerRequestError> {
    let workplace = app_state
        .repositories
        .workplace()
        .find(&current_user, path.workplace_id)
        .await?;

    let target_month = TargetMonth::new_with_default_timezone(params.year, params.month);
    let finder = AttendancesForMonth::new(&app_state, &workplace, &target_month);
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
    app_state: Data<AppState>,
    current_user: ReqData<User>,
    path: Path<PathInfo>,
    form: Form<CreationParameters>,
) -> Result<HttpResponse, PerRequestError> {
    let workplace = app_state
        .repositories
        .workplace()
        .find(&current_user, path.workplace_id)
        .await?;

    let repository = app_state.repositories.attendance_record();
    let attendance_record = repository
        .create(&workplace, &form.event, &form.datetime.to_utc().into())
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
    app_state: Data<AppState>,
    current_user: ReqData<User>,
    path: Path<DestroyPath>,
) -> Result<HttpResponse, PerRequestError> {
    let workplace = app_state
        .repositories
        .workplace()
        .find(&current_user, path.workplace_id)
        .await?;

    let repository = app_state.repositories.attendance_record();
    repository.destroy(&workplace, path.id).await?;

    let response = HttpResponse::Ok().finish();
    Ok(response)
}
