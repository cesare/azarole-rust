use actix_session::Session;
use actix_web::{
    HttpResponse,
    web::{Data, ServiceConfig, post},
};

use crate::{
    context::ApplicationContext,
    errors::PerRequestError,
};

pub fn routes(config: &mut ServiceConfig) {
    config
        .route("", post().to(request_authentication))
        .route("/callback", post().to(callback));
}

#[allow(unused_variables)]
async fn request_authentication(context: Data<ApplicationContext>, session: Session) -> Result<HttpResponse, PerRequestError> {
    todo!()
}

#[allow(unused_variables)]
async fn callback(context: Data<ApplicationContext>, session: Session) -> Result<HttpResponse, PerRequestError> {
    todo!()
}
