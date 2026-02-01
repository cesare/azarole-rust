use actix_web::{App, http::StatusCode, test, web::Data};
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
