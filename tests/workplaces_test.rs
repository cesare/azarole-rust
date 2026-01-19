use actix_web::{App, http::StatusCode, test, web::Data};
use azarole::{models::User, repositories::RepositoryFactory};
use serde::Serialize;
use serde_json::{Value, json};
use sqlx::SqlitePool;

mod common;

#[sqlx::test(fixtures("users"))]
async fn workplace_listing_without_signin(pool: SqlitePool) {
    let context = common::create_context(pool);
    let app = test::init_service(
        App::new()
            .wrap(common::create_session_middleware())
            .app_data(Data::new(context))
            .configure(azarole::handlers::routes),
    )
    .await;

    let request = test::TestRequest::get().uri("/workplaces").to_request();
    let result = test::try_call_service(&app, request).await;

    assert!(result.is_err());

    let error = result.unwrap_err();
    let response = error.error_response();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[sqlx::test(fixtures("users", "workplaces"))]
async fn workplace_listing(pool: SqlitePool) {
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
        .uri("/workplaces")
        .insert_header(("Cookie", cookie_value))
        .to_request();
    let response = test::call_service(&app, request).await;

    assert!(response.status().is_success());

    let response_json: Value = test::read_body_json(response).await;
    let expected_json = json!({
        "workplaces": [
            {
                "id": 1,
                "name": "workplace-01-for-user-01",
            },
            {
                "id": 2,
                "name": "workplace-02-for-user-01",
            },
        ],
    });
    assert_eq!(response_json, expected_json);
}

#[sqlx::test(fixtures("users"))]
async fn workplace_creation_without_signin(pool: SqlitePool) {
    let context = common::create_context(pool);
    let app = test::init_service(
        App::new()
            .wrap(common::create_session_middleware())
            .app_data(Data::new(context))
            .configure(azarole::handlers::routes),
    )
    .await;

    #[derive(Serialize)]
    struct Params {
        name: String,
    }

    let request = test::TestRequest::post()
        .uri("/workplaces")
        .set_form(Params {
            name: "test-workplace".to_owned(),
        })
        .to_request();
    let result = test::try_call_service(&app, request).await;

    assert!(result.is_err());

    let error = result.unwrap_err();
    let response = error.error_response();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[sqlx::test(fixtures("users"))]
async fn workplace_creation(pool: SqlitePool) {
    let context = common::create_context(pool);
    let app = test::init_service(
        App::new()
            .wrap(common::create_session_middleware())
            .app_data(Data::new(context.clone()))
            .configure(azarole::handlers::routes),
    )
    .await;

    #[derive(Serialize)]
    struct Params {
        name: String,
    }

    let cookie_value = common::generate_cookie_value_with_signin_user(1);
    let request = test::TestRequest::post()
        .uri("/workplaces")
        .insert_header(("Cookie", cookie_value))
        .set_form(Params {
            name: "test-workplace".to_owned(),
        })
        .to_request();
    let response = test::call_service(&app, request).await;

    assert_eq!(response.status(), StatusCode::CREATED);

    let response_json: Value = test::read_body_json(response).await;
    let expected_json = json!({
        "workplace": {
            "id": 1,
            "name": "test-workplace",
        },
    });
    assert_eq!(response_json, expected_json);

    let user = User { id: 1.into() };
    let repository = context.repositories.workplace();
    let workplaces = repository.list(&user).await.unwrap();
    assert_eq!(workplaces.iter().count(), 1);

    let workplace = workplaces.first().unwrap();
    assert_eq!(workplace.name, "test-workplace");
}
