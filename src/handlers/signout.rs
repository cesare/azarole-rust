use actix_session::Session;
use actix_web::{
    HttpResponse,
    web::{ServiceConfig, delete},
};

use crate::errors::PerRequestError;

pub(super) fn routes(config: &mut ServiceConfig) {
    config.route("", delete().to(signout));
}

async fn signout(session: Session) -> Result<HttpResponse, PerRequestError> {
    session.clear();
    session.renew();

    let response = HttpResponse::Ok().finish();
    Ok(response)
}
