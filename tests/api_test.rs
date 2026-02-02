use actix_web::{App, http::StatusCode, test, web::Data};
use azarole::models::{AttendanceRecord, attendance_record};
use sqlx::SqlitePool;

mod common;

#[sqlx::test(fixtures("users", "workplaces"))]
async fn clock_in_without_api_key(pool: SqlitePool) {
    let context = common::create_context(pool);
    let app = test::init_service(
        App::new()
            .wrap(common::create_session_middleware())
            .app_data(Data::new(context))
            .configure(azarole::handlers::routes),
    )
    .await;

    let request = test::TestRequest::post()
        .uri("/api/workplaces/1/clock_ins")
        .to_request();
    let result = test::try_call_service(&app, request).await;
    assert!(result.is_err());

    let error = result.unwrap_err();
    let response = error.error_response();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[sqlx::test(fixtures("users", "workplaces", "api_keys"))]
async fn clock_in_with_api_key(pool: SqlitePool) {
    let context = common::create_context(pool.clone());
    let app = test::init_service(
        App::new()
            .wrap(common::create_session_middleware())
            .app_data(Data::new(context))
            .configure(azarole::handlers::routes),
    )
    .await;

    let valid_api_key = "AAECAwQFBgcICQoLDA0ODxAREhMUFRYXGBkaGxwdHh8gISIjJCUmJygpKissLS4vMDEyMzQ1Njc4OTo7PD0-P0BBQkNERUZHSElKS0xNTk9QUVJTVFVWV1hZWltcXV5f";
    let request = test::TestRequest::post()
        .uri("/api/workplaces/1/clock_ins")
        .insert_header(("Authorization", format!("Bearer {valid_api_key}")))
        .to_request();
    let response = test::call_service(&app, request).await;

    assert_eq!(response.status(), StatusCode::CREATED);

    let records: Vec<AttendanceRecord> = sqlx::query_as("select id, workplace_id, event, recorded_at from attendance_records where workplace_id = 1 order by recorded_at desc")
        .fetch_all(&pool)
        .await.unwrap();
    assert_eq!(records.iter().count(), 1);

    let attendance = records.first().unwrap();
    assert_eq!(attendance.event, attendance_record::Event::ClockIn);
}

#[sqlx::test(fixtures("users", "workplaces", "api_keys"))]
async fn clock_in_with_invalid_api_key(pool: SqlitePool) {
    let context = common::create_context(pool.clone());
    let app = test::init_service(
        App::new()
            .wrap(common::create_session_middleware())
            .app_data(Data::new(context))
            .configure(azarole::handlers::routes),
    )
    .await;

    let invalid_api_key = "invalid-api-key";
    let request = test::TestRequest::post()
        .uri("/api/workplaces/1/clock_ins")
        .insert_header(("Authorization", format!("Bearer {invalid_api_key}")))
        .to_request();
    let result = test::try_call_service(&app, request).await;
    assert!(result.is_err());

    let error = result.unwrap_err();
    let response = error.error_response();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

    let records: Vec<AttendanceRecord> = sqlx::query_as("select id, workplace_id, event, recorded_at from attendance_records where workplace_id = $1")
        .fetch_all(&pool)
        .await.unwrap();
    assert_eq!(records.iter().count(), 0);
}

#[sqlx::test(fixtures("users", "workplaces"))]
async fn clock_out_without_api_key(pool: SqlitePool) {
    let context = common::create_context(pool);
    let app = test::init_service(
        App::new()
            .wrap(common::create_session_middleware())
            .app_data(Data::new(context))
            .configure(azarole::handlers::routes),
    )
    .await;

    let request = test::TestRequest::post()
        .uri("/api/workplaces/1/clock_outs")
        .to_request();
    let result = test::try_call_service(&app, request).await;
    assert!(result.is_err());

    let error = result.unwrap_err();
    let response = error.error_response();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[sqlx::test(fixtures("users", "workplaces", "api_keys"))]
async fn clock_out_with_api_key(pool: SqlitePool) {
    let context = common::create_context(pool.clone());
    let app = test::init_service(
        App::new()
            .wrap(common::create_session_middleware())
            .app_data(Data::new(context))
            .configure(azarole::handlers::routes),
    )
    .await;

    let valid_api_key = "AAECAwQFBgcICQoLDA0ODxAREhMUFRYXGBkaGxwdHh8gISIjJCUmJygpKissLS4vMDEyMzQ1Njc4OTo7PD0-P0BBQkNERUZHSElKS0xNTk9QUVJTVFVWV1hZWltcXV5f";
    let request = test::TestRequest::post()
        .uri("/api/workplaces/1/clock_outs")
        .insert_header(("Authorization", format!("Bearer {valid_api_key}")))
        .to_request();
    let response = test::call_service(&app, request).await;

    assert_eq!(response.status(), StatusCode::CREATED);

    let records: Vec<AttendanceRecord> = sqlx::query_as("select id, workplace_id, event, recorded_at from attendance_records where workplace_id = 1 order by recorded_at desc")
        .fetch_all(&pool)
        .await.unwrap();
    assert_eq!(records.iter().count(), 1);

    let attendance = records.first().unwrap();
    assert_eq!(attendance.event, attendance_record::Event::ClockOut);
}

#[sqlx::test(fixtures("users", "workplaces", "api_keys"))]
async fn clock_out_with_invalid_api_key(pool: SqlitePool) {
    let context = common::create_context(pool.clone());
    let app = test::init_service(
        App::new()
            .wrap(common::create_session_middleware())
            .app_data(Data::new(context))
            .configure(azarole::handlers::routes),
    )
    .await;

    let invalid_api_key = "invalid-api-key";
    let request = test::TestRequest::post()
        .uri("/api/workplaces/1/clock_outs")
        .insert_header(("Authorization", format!("Bearer {invalid_api_key}")))
        .to_request();
    let result = test::try_call_service(&app, request).await;
    assert!(result.is_err());

    let error = result.unwrap_err();
    let response = error.error_response();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

    let records: Vec<AttendanceRecord> = sqlx::query_as("select id, workplace_id, event, recorded_at from attendance_records where workplace_id = $1")
        .fetch_all(&pool)
        .await.unwrap();
    assert_eq!(records.iter().count(), 0);
}
