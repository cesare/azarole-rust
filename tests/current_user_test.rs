use actix_web::{App, http::StatusCode, test, web::Data};
use serde_json::{Value, json};
use sqlx::SqlitePool;

mod common;

#[sqlx::test(fixtures("users"))]
async fn current_user_without_signing_in(pool: SqlitePool) {
    let context = common::create_context(pool);
    let app = test::init_service(
        App::new()
            .wrap(common::create_session_middleware())
            .app_data(Data::new(context))
            .configure(azarole::handlers::routes),
    )
    .await;

    let request = test::TestRequest::get().uri("/current_user").to_request();
    let result = test::try_call_service(&app, request).await;

    assert!(result.is_err());

    let error = result.unwrap_err();
    let response = error.error_response();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[sqlx::test(fixtures("users"))]
async fn current_user_with_signing_in(pool: SqlitePool) {
    let context = common::create_context(pool);
    let app = test::init_service(
        App::new()
            .wrap(common::create_session_middleware())
            .app_data(Data::new(context))
            .configure(azarole::handlers::routes),
    )
    .await;

    let cookie_value = common::generate_cookie_value_with_signin_user(1);
    let request = test::TestRequest::get()
        .uri("/current_user")
        .insert_header(("Cookie", cookie_value))
        .to_request();
    let response = test::call_service(&app, request).await;

    assert!(response.status().is_success());

    let response_json: Value = test::read_body_json(response).await;
    let expected_json = json!({
        "user": {
            "id": 1,
        },
    });
    assert_eq!(response_json, expected_json);
}
