use actix_web::{
    HttpResponse,
    web::{Data, Form, Path, ReqData, ServiceConfig, delete, get, post},
};
use serde::Deserialize;
use serde_json::json;

use super::views::ApiKeyView;
use crate::{
    context::AppState,
    errors::PerRequestError,
    models::{ApiKeyId, User},
    repositories::RepositoryFactory,
};

mod registration;
use registration::ApiKeyRegistration;

pub(super) fn routes(config: &mut ServiceConfig) {
    config
        .route("", get().to(index))
        .route("", post().to(create))
        .route("/{id}", delete().to(destroy));
}

async fn index(
    app_state: Data<AppState>,
    current_user: ReqData<User>,
) -> Result<HttpResponse, PerRequestError> {
    let repository = app_state.repositories.api_key();
    let api_keys = repository.list(&current_user).await?;

    let api_key_views = api_keys
        .iter()
        .map(ApiKeyView::new)
        .collect::<Vec<ApiKeyView>>();
    let response_json = json!({
        "api_keys":  api_key_views,
        "apiKeys":  api_key_views,
    });
    let response = HttpResponse::Ok().json(response_json);
    Ok(response)
}

#[derive(Deserialize)]
struct CreatingApiKeyForm {
    name: String,
}

async fn create(
    app_state: Data<AppState>,
    current_user: ReqData<User>,
    form: Form<CreatingApiKeyForm>,
) -> Result<HttpResponse, PerRequestError> {
    let registration = ApiKeyRegistration::new(&app_state, &current_user, &form.name);
    let registration_details = registration.execute().await?;

    let response_json = json!({
        "api_key": registration_details,
        "apiKey": registration_details,
    });
    let response = HttpResponse::Ok().json(response_json);
    Ok(response)
}

#[derive(Deserialize)]
struct ApiKeyPath {
    id: ApiKeyId,
}

async fn destroy(
    app_state: Data<AppState>,
    current_user: ReqData<User>,
    path: Path<ApiKeyPath>,
) -> Result<HttpResponse, PerRequestError> {
    let repository = app_state.repositories.api_key();
    repository.destroy(&current_user, &path.id).await?;

    let response = HttpResponse::Ok().finish();
    Ok(response)
}
