use actix_web::{
    HttpResponse,
    web::{ReqData, ServiceConfig, get},
};
use serde_json::json;

use super::views::UserView;
use crate::{errors::PerRequestError, models::User};

pub(super) fn routes(config: &mut ServiceConfig) {
    config.route("", get().to(current_user));
}

async fn current_user(user: ReqData<User>) -> Result<HttpResponse, PerRequestError> {
    let response_json = json!({
        "user": UserView::new(&user),
    });
    let response = HttpResponse::Ok().json(response_json);
    Ok(response)
}
