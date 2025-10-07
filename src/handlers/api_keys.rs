use actix_web::{
    web::{delete, get, Data, Path, ReqData, ServiceConfig},
    HttpResponse
};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{
    context::ApplicationContext,
    errors::PerRequestError,
    models::{ApiKey, ApiKeyId, ApiKeyResources, User},
};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ApiKeyView<'a> {
    id: &'a ApiKeyId,
    name: &'a String,
}

impl<'a> ApiKeyView<'a> {
    fn new(api_key: &'a ApiKey) -> Self {
        Self {
            id: &api_key.id,
            name: &api_key.name,
        }
    }
}

pub(super) fn routes(config: &mut ServiceConfig) {
    config
        .route("", get().to(index))
        .route("/{id}", delete().to(destroy));
}

async fn index(context: Data<ApplicationContext>, current_user: ReqData<User>) -> Result<HttpResponse, PerRequestError> {
    let resources = ApiKeyResources::new(&context, &current_user);
    let api_keys = resources.list().await?;

    let response_json = json!({
        "api_keys": api_keys.iter().map(ApiKeyView::new).collect::<Vec<ApiKeyView>>(),
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
