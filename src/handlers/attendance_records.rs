use actix_web::{
    web::{delete, get, post, Data, Form, Path, Query, ReqData, ServiceConfig},
    HttpResponse
};
use chrono::{DateTime, Local};
use serde::{Deserialize};
use serde_json::json;

use crate::{
    context::ApplicationContext, errors::PerRequestError, models::{
        attendance_record, AttendanceRecordId, AttendanceRecordResources, User, WorkplaceId, WorkplaceResources
    }
};
use super::views::{AttendanceRecordView, WorkplaceView};

mod listing;
use listing::{TargetMonth, AttendancesForMonth, Year, Month};

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

#[derive(Deserialize)]
struct IndexParameters {
    #[serde(default)]
    year: Year,
    #[serde(default)]
    month: Month,
}

async fn index(context: Data<ApplicationContext>, current_user: ReqData<User>, path: Path<PathInfo>, params: Query<IndexParameters>) -> Result<HttpResponse, PerRequestError> {
    let workplace = WorkplaceResources::new(&context, &current_user).find(path.workplace_id).await?;

    let target_month = TargetMonth::new(&params.year, &params.month);
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

async fn create(context: Data<ApplicationContext>, current_user: ReqData<User>, path: Path<PathInfo>, form: Form<CreationParameters>) -> Result<HttpResponse, PerRequestError> {
    let workplace = WorkplaceResources::new(&context, &current_user).find(path.workplace_id).await?;

    let resources = AttendanceRecordResources::new(&context, &workplace);
    let attendance_record = resources.create(&form.event, &form.datetime.to_utc()).await?;

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

async fn destroy(context: Data<ApplicationContext>, current_user: ReqData<User>, path: Path<DestroyPath>) -> Result<HttpResponse, PerRequestError> {
    let workplace = WorkplaceResources::new(&context, &current_user).find(path.workplace_id).await?;

    let resources = AttendanceRecordResources::new(&context, &workplace);
    resources.destroy(path.id).await?;

    let response = HttpResponse::Ok().finish();
    Ok(response)
}
