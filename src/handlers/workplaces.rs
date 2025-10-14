use actix_web::{
    web::{Data, Form, ReqData, ServiceConfig, get, post},
    HttpResponse
};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{
    context::ApplicationContext,
    errors::PerRequestError,
    models::{
        User, Workplace, WorkplaceId, WorkplaceResources
    },
};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(in crate::handlers) struct WorkplaceView<'a> {
    id: &'a WorkplaceId,
    name: &'a String,
}

impl<'a> WorkplaceView<'a> {
    pub(in crate::handlers) fn new(workplace: &'a Workplace) -> Self {
        Self {
            id: &workplace.id,
            name: &workplace.name,
        }
    }
}

pub(super) fn routes(config: &mut ServiceConfig) {
    config
        .route("", get().to(index))
        .route("", post().to(create));
}

async fn index(context: Data<ApplicationContext>, current_user: ReqData<User>) -> Result<HttpResponse, PerRequestError> {
    let resources = WorkplaceResources::new(&context, &current_user);
    let workplaces = resources.list().await?;

    let response_json = json!({
        "workplaces": workplaces.iter().map(WorkplaceView::new).collect::<Vec<WorkplaceView>>(),
    });
    let response = HttpResponse::Ok().json(response_json);
    Ok(response)
}

#[derive(Deserialize)]
struct CreatingWorkplaceForm {
    name: String,
}

async fn create(context: Data<ApplicationContext>, current_user: ReqData<User>, form: Form<CreatingWorkplaceForm>)  -> Result<HttpResponse, PerRequestError> {
    let resources = WorkplaceResources::new(&context, &current_user);
    let workpalce = resources.create(&form.name).await?;

    let response_json = json!({
        "workplace": WorkplaceView::new(&workpalce),
    });
    let response = HttpResponse::Created().json(response_json);
    Ok(response)
}
