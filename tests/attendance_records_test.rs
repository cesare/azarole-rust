use actix_web::{App, http::StatusCode, test, web::Data};
use azarole::models::{AttendanceRecord, attendance_record};
use chrono::{DateTime, Local};
use serde::Serialize;
use serde_json::{Value, json};
use sqlx::SqlitePool;

mod common;

#[derive(Serialize)]
struct CreationParams {
    event: attendance_record::Event,
    datetime: DateTime<Local>,
}

#[sqlx::test(fixtures("users"))]
async fn attendance_record_creation_without_signin(pool: SqlitePool) {
    let app_state = common::create_app_state(pool);
    let app = test::init_service(
        App::new()
            .wrap(common::create_session_middleware())
            .app_data(Data::new(app_state))
            .configure(azarole::handlers::routes),
    )
    .await;

    let request = test::TestRequest::post()
        .uri("/workplaces/1/attendance_records")
        .set_form(CreationParams {
            event: attendance_record::Event::ClockIn,
            datetime: Local::now(),
        })
        .to_request();
    let result = test::try_call_service(&app, request).await;

    assert!(result.is_err());

    let error = result.unwrap_err();
    let response = error.error_response();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[sqlx::test(fixtures("users", "workplaces"))]
async fn attendance_record_creation(pool: SqlitePool) {
    let app_state = common::create_app_state(pool.clone());
    let app = test::init_service(
        App::new()
            .wrap(common::create_session_middleware())
            .app_data(Data::new(app_state))
            .configure(azarole::handlers::routes),
    )
    .await;

    let now = Local::now();
    let cookie_value = common::generate_cookie_value_with_signin_user(1);
    let request = test::TestRequest::post()
        .uri("/workplaces/1/attendance_records")
        .insert_header(("Cookie", cookie_value))
        .set_form(CreationParams {
            event: attendance_record::Event::ClockIn,
            datetime: now,
        })
        .to_request();
    let response = test::call_service(&app, request).await;
    assert_eq!(response.status(), StatusCode::OK);

    let response_json: Value = test::read_body_json(response).await;
    let expected_json = json!({
        "attendanceRecord": {
            "id": 1,
            "event": "clock-in",
            "recordedAt": now.to_utc(),
        },
    });
    assert_eq!(response_json, expected_json);

    let attendance_records: Vec<AttendanceRecord> = sqlx::query_as("select id, workplace_id, event, recorded_at from attendance_records where workplace_id = $1 order by created_at desc")
        .bind(1)
        .fetch_all(&pool)
        .await
        .unwrap();
    assert_eq!(attendance_records.iter().count(), 1);

    let attendance_record = attendance_records.first().unwrap();
    assert_eq!(attendance_record.event, attendance_record::Event::ClockIn);
    assert_eq!(attendance_record.recorded_at, now.to_utc().into());
}

#[sqlx::test(fixtures("users", "workplaces"))]
async fn attendance_record_creation_with_other_user(pool: SqlitePool) {
    let app_state = common::create_app_state(pool.clone());
    let app = test::init_service(
        App::new()
            .wrap(common::create_session_middleware())
            .app_data(Data::new(app_state))
            .configure(azarole::handlers::routes),
    )
    .await;

    let now = Local::now();
    let cookie_value = common::generate_cookie_value_with_signin_user(2);
    let request = test::TestRequest::post()
        .uri("/workplaces/1/attendance_records")
        .insert_header(("Cookie", cookie_value))
        .set_form(CreationParams {
            event: attendance_record::Event::ClockIn,
            datetime: now,
        })
        .to_request();
    let response = test::call_service(&app, request).await;
    assert_eq!(response.status(), StatusCode::NOT_FOUND);

    let attendance_records: Vec<AttendanceRecord> = sqlx::query_as("select id, workplace_id, event, recorded_at from attendance_records where workplace_id = $1 order by created_at desc")
        .bind(1)
        .fetch_all(&pool)
        .await
        .unwrap();
    assert_eq!(attendance_records.iter().count(), 0);
}

#[sqlx::test(fixtures("users", "workplaces", "attendance_records"))]
async fn attendance_record_deletion_without_signin(pool: SqlitePool) {
    let app_state = common::create_app_state(pool.clone());
    let app = test::init_service(
        App::new()
            .wrap(common::create_session_middleware())
            .app_data(Data::new(app_state))
            .configure(azarole::handlers::routes),
    )
    .await;

    let request = test::TestRequest::delete()
        .uri("/workplaces/1/attendance_records/1")
        .to_request();
    let result = test::try_call_service(&app, request).await;
    assert!(result.is_err());

    let error = result.unwrap_err();
    let response = error.error_response();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

    let attendance_record: Option<AttendanceRecord> = sqlx::query_as(
        "select id, workplace_id, event, recorded_at from attendance_records where id = $1",
    )
    .bind(1)
    .fetch_optional(&pool)
    .await
    .unwrap();

    assert!(attendance_record.is_some());
}

#[sqlx::test(fixtures("users", "workplaces", "attendance_records"))]
async fn attendance_record_deletion(pool: SqlitePool) {
    let app_state = common::create_app_state(pool.clone());
    let app = test::init_service(
        App::new()
            .wrap(common::create_session_middleware())
            .app_data(Data::new(app_state))
            .configure(azarole::handlers::routes),
    )
    .await;

    let cookie_value = common::generate_cookie_value_with_signin_user(1);
    let request = test::TestRequest::delete()
        .uri("/workplaces/1/attendance_records/1")
        .insert_header(("Cookie", cookie_value))
        .to_request();
    let response = test::call_service(&app, request).await;
    assert_eq!(response.status(), StatusCode::OK);

    let attendance_record: Option<AttendanceRecord> = sqlx::query_as(
        "select id, workplace_id, event, recorded_at from attendance_records where id = $1",
    )
    .bind(1)
    .fetch_optional(&pool)
    .await
    .unwrap();

    assert!(attendance_record.is_none());
}

#[sqlx::test(fixtures("users", "workplaces", "attendance_records"))]
async fn attendance_record_deletion_with_other_user(pool: SqlitePool) {
    let app_state = common::create_app_state(pool.clone());
    let app = test::init_service(
        App::new()
            .wrap(common::create_session_middleware())
            .app_data(Data::new(app_state))
            .configure(azarole::handlers::routes),
    )
    .await;

    let cookie_value = common::generate_cookie_value_with_signin_user(2);
    let request = test::TestRequest::delete()
        .uri("/workplaces/1/attendance_records/1")
        .insert_header(("Cookie", cookie_value))
        .to_request();
    let response = test::call_service(&app, request).await;
    assert_eq!(response.status(), StatusCode::NOT_FOUND);

    let attendance_record: Option<AttendanceRecord> = sqlx::query_as(
        "select id, workplace_id, event, recorded_at from attendance_records where id = $1",
    )
    .bind(1)
    .fetch_optional(&pool)
    .await
    .unwrap();

    assert!(attendance_record.is_some());
}
