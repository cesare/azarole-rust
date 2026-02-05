use actix_web::{App, http::StatusCode, test, web::Data};
use azarole::models::ApiKey;
use serde_json::{Value, json};
use sqlx::SqlitePool;

mod common;

#[sqlx::test(fixtures("users", "api_keys"))]
async fn listing_api_keys_without_signin(pool: SqlitePool) {
    let context = common::create_context(pool);
    let app = test::init_service(
        App::new()
            .wrap(common::create_session_middleware())
            .app_data(Data::new(context))
            .configure(azarole::handlers::routes),
    )
    .await;

    let request = test::TestRequest::get().uri("/api_keys").to_request();
    let result = test::try_call_service(&app, request).await;

    assert!(result.is_err());

    let error = result.unwrap_err();
    let response = error.error_response();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[sqlx::test(fixtures("users", "api_keys"))]
async fn listing_api_keys(pool: SqlitePool) {
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
        .uri("/api_keys")
        .insert_header(("Cookie", cookie_value))
        .to_request();
    let response = test::call_service(&app, request).await;

    assert!(response.status().is_success());

    let response_json: Value = test::read_body_json(response).await;
    let expected_json = json!({
        "apiKeys": [
            {
                "id": 1,
                "name": "test-api-key-01",
                "createdAt": "2026-02-02T01:02:03Z",
            },
        ],
        // deprecated
        "api_keys": [
            {
                "id": 1,
                "name": "test-api-key-01",
                "createdAt": "2026-02-02T01:02:03Z",
            },
        ],
    });
    assert_eq!(response_json, expected_json);
}

#[sqlx::test(fixtures("users", "api_keys"))]
async fn deleting_api_key_without_signin(pool: SqlitePool) {
    let context = common::create_context(pool.clone());
    let app = test::init_service(
        App::new()
            .wrap(common::create_session_middleware())
            .app_data(Data::new(context))
            .configure(azarole::handlers::routes),
    )
    .await;

    let request = test::TestRequest::delete().uri("/api_keys/1").to_request();
    let result = test::try_call_service(&app, request).await;

    assert!(result.is_err());

    let error = result.unwrap_err();
    let response = error.error_response();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

    let result: Option<ApiKey> =
        sqlx::query_as("select id, user_id, name, digest, created_at from api_keys where id = $1")
            .bind(1)
            .fetch_optional(&pool)
            .await
            .unwrap();
    assert!(result.is_some());
}

#[sqlx::test(fixtures("users", "api_keys"))]
async fn deleting_api_key(pool: SqlitePool) {
    let context = common::create_context(pool.clone());
    let app = test::init_service(
        App::new()
            .wrap(common::create_session_middleware())
            .app_data(Data::new(context))
            .configure(azarole::handlers::routes),
    )
    .await;

    let cookie_value = common::generate_cookie_value_with_signin_user(1);
    let request = test::TestRequest::delete()
        .uri("/api_keys/1")
        .insert_header(("Cookie", cookie_value))
        .to_request();
    let response = test::call_service(&app, request).await;

    assert!(response.status().is_success());

    let result: Option<ApiKey> =
        sqlx::query_as("select id, user_id, name, digest, created_at from api_keys where id = $1")
            .bind(1)
            .fetch_optional(&pool)
            .await
            .unwrap();
    assert!(result.is_none());
}
