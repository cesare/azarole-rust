use actix_web::{
    web::{Data, ReqData, ServiceConfig, get},
    HttpResponse
};
use serde::Serialize;
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
struct WorkplaceView<'a> {
    id: &'a WorkplaceId,
    name: &'a String,
}

impl<'a> WorkplaceView<'a> {
    fn new(workplace: &'a Workplace) -> Self {
        Self {
            id: &workplace.id,
            name: &workplace.name,
        }
    }
}

pub(super) fn routes(config: &mut ServiceConfig) {
    config
        .route("", get().to(index));
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
