use actix_web::{App, http::StatusCode, test, web::Data};
use azarole::models::{ApiKey, TokenDigester};
use serde::Serialize;
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

#[derive(Serialize)]
struct CreatingApiKeyForm {
    name: String,
}

#[sqlx::test(fixtures("users"))]
async fn creating_api_key_without_signin(pool: SqlitePool) {
    let context = common::create_context(pool.clone());
    let app = test::init_service(
        App::new()
            .wrap(common::create_session_middleware())
            .app_data(Data::new(context))
            .configure(azarole::handlers::routes),
    )
    .await;

    let params = CreatingApiKeyForm {
        name: "testing".to_owned(),
    };
    let request = test::TestRequest::post()
        .uri("/api_keys")
        .set_form(&params)
        .to_request();
    let result = test::try_call_service(&app, request).await;

    assert!(result.is_err());

    let error = result.unwrap_err();
    let response = error.error_response();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

    let existing_keys: Vec<ApiKey> =
        sqlx::query_as("select id, user_id, name, digest, created_at from api_keys")
            .bind(1)
            .fetch_all(&pool)
            .await
            .unwrap();
    assert_eq!(existing_keys.iter().count(), 0);
}

#[sqlx::test(fixtures("users"))]
async fn creating_api_key(pool: SqlitePool) {
    let context = common::create_context(pool.clone());
    let app = test::init_service(
        App::new()
            .wrap(common::create_session_middleware())
            .app_data(Data::new(context.clone()))
            .configure(azarole::handlers::routes),
    )
    .await;

    let cookie_value = common::generate_cookie_value_with_signin_user(1);
    let params = CreatingApiKeyForm {
        name: "testing".to_owned(),
    };
    let request = test::TestRequest::post()
        .uri("/api_keys")
        .insert_header(("Cookie", cookie_value))
        .set_form(&params)
        .to_request();
    let response = test::call_service(&app, request).await;

    assert_eq!(response.status(), StatusCode::OK);

    let created_key: ApiKey =
        sqlx::query_as("select id, user_id, name, digest, created_at from api_keys where user_id = $1 order by created_at limit 1")
            .bind(1)
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(created_key.name, "testing");

    let response_json: Value = test::read_body_json(response).await;
    let api_key_node = response_json.get("apiKey").unwrap();
    let name = api_key_node.get("name").unwrap().as_str().unwrap();
    assert_eq!(created_key.name, name);

    let token = api_key_node.get("token").unwrap().as_str().unwrap();
    let digester = TokenDigester::new(&context.secrets.api_key.digesting_secret_key);
    let expected_digest = digester.digest_token(token).unwrap();
    assert_eq!(created_key.digest, expected_digest);
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
