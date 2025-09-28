use std::ops::Deref;

use actix_web::web::{post, Data, Path, ReqData, ServiceConfig};
use actix_web::{HttpResponse, Result};
use chrono::Utc;
use serde_json::json;

use crate::context::ApplicationContext;
use crate::errors::{DatabaseError, PerRequestError};
use crate::models::attendance_record::AttendanceRecord;
use crate::models::user::User;

pub fn routes(config: &mut ServiceConfig) {
    config
        .route("/workplaces/{workplace_id}/clock_ins", post().to(clock_in));
}

async fn clock_in(context: Data<ApplicationContext>, current_user: ReqData<User>, path: Path<u32>) -> Result<HttpResponse, PerRequestError> {
    let workplace_id = path.into_inner();
    ensure_workplace(context.get_ref(), current_user.deref(), workplace_id).await?;

    let attendance_record = create_clock_in(context.get_ref(), workplace_id).await?;
    let response_json = json!({
        "attendanceRecord": attendance_record,
    });
    let response = HttpResponse::Created().json(response_json);
    Ok(response)
}

async fn ensure_workplace(context: &ApplicationContext, user: &User, workplace_id: u32) -> Result<(), DatabaseError> {
    let result = sqlx::query_as::<sqlx::Sqlite, (u32,)>("select 1 from workplaces where user_id = $1 and id = $2")
        .bind(user.id)
        .bind(workplace_id)
        .fetch_one(&context.database.pool)
        .await;
    match result {
        Ok(_) => Ok(()),
        Err(error) => Err(error.into()),
    }
}

async fn create_clock_in(context: &ApplicationContext, workplace_id: u32) -> Result<AttendanceRecord, DatabaseError> {
    let now = Utc::now();
    let statement = "insert into attendance_records (workplace_id, event, recorded_at, created_at) values($1, $2, $3, $4) returning id, workplace_id, event, recorded_at";
    let result = sqlx::query_as::<sqlx::Sqlite, AttendanceRecord>(statement)
        .bind(workplace_id)
        .bind("clock-in")
        .bind(now)
        .bind(now)
        .fetch_one(&context.database.pool)
        .await;
    match result {
        Ok(attendance_record) => Ok(attendance_record),
        Err(error) => Err(error.into()),
    }
}
