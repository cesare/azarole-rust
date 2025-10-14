use actix_web::{
    web::{delete, get, post, Data, Form, Path, Query, ReqData, ServiceConfig},
    HttpResponse
};
use chrono::{DateTime, Datelike, Local};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{
    context::ApplicationContext, errors::PerRequestError, models::{
        attendance_record, AttendanceRecordId, AttendanceRecordResources, User, WorkplaceId, WorkplaceResources
    }
};
use super::views::{AttendanceRecordView, WorkplaceView};

mod listing;
use listing::{TargetMonth, AttendancesForMonth};

pub(super) fn routes(config: &mut ServiceConfig) {
    config
        .route("", get().to(index))
        .route("", post().to(create))
        .route("/attendance_records/{id}", delete().to(destroy));
}

#[derive(Clone, Copy, Deserialize, Serialize)]
#[repr(transparent)]
struct Year(i32);

impl Default for Year {
    fn default() -> Self {
        Self(Local::now().year())
    }
}

impl From<Year> for i32 {
    fn from(value: Year) -> Self {
        value.0
    }
}

#[derive(Clone, Copy, Deserialize, Serialize)]
#[repr(transparent)]
struct Month(u32);

impl Default for Month {
    fn default() -> Self {
        Self(Local::now().month())
    }
}

impl From<Month> for u32 {
    fn from(value: Month) -> Self {
        value.0
    }
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

    let target_month = TargetMonth::new(params.year.into(), params.month.into());
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
