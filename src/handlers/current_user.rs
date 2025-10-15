use actix_web::{
    web::{get, ReqData, ServiceConfig}, HttpResponse
};
use serde_json::json;

use crate::{
    errors::PerRequestError,
    models::User,
};
use super::views::UserView;

pub(super) fn routes(config: &mut ServiceConfig) {
    config
        .route("", get().to(current_user));
}

async fn current_user(user: ReqData<User>) -> Result<HttpResponse, PerRequestError> {
    let response_json = json!({
        "user": UserView::new(&user),
    });
    let response = HttpResponse::Ok().json(response_json);
    Ok(response)
}
