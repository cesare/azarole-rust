use actix_web::{
    web::{delete, get, post, Data, Form, Path, ReqData, ServiceConfig},
    HttpResponse
};
use serde::Deserialize;
use serde_json::json;

use crate::{
    context::ApplicationContext,
    errors::PerRequestError,
    models::{ApiKeyId, ApiKeyResources, User},
};
use super::views::ApiKeyView;

mod registration;
use registration::ApiKeyRegistration;

pub(super) fn routes(config: &mut ServiceConfig) {
    config
        .route("", get().to(index))
        .route("", post().to(create))
        .route("/{id}", delete().to(destroy));
}

async fn index(context: Data<ApplicationContext>, current_user: ReqData<User>) -> Result<HttpResponse, PerRequestError> {
    let resources = ApiKeyResources::new(&context, &current_user);
    let api_keys = resources.list().await?;

    let api_key_views = api_keys.iter().map(ApiKeyView::new).collect::<Vec<ApiKeyView>>();
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

async fn create(context: Data<ApplicationContext>, current_user: ReqData<User>, form: Form<CreatingApiKeyForm>) -> Result<HttpResponse, PerRequestError> {
    let registration = ApiKeyRegistration::new(&context, &current_user, &form.name);
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

async fn destroy(context: Data<ApplicationContext>, current_user: ReqData<User>, path: Path<ApiKeyPath>) -> Result<HttpResponse, PerRequestError> {
    let resources = ApiKeyResources::new(&context, &current_user);
    resources.destroy(&path.id).await?;

    let response = HttpResponse::Ok().finish();
    Ok(response)
}
