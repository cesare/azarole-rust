use actix_session::Session;
use actix_web::{
    HttpResponse,
    http::header,
    web::{Data, ServiceConfig, post},
};

use crate::{
    context::ApplicationContext,
    errors::PerRequestError,
};

mod authentication_request;
use authentication_request::AuthenticationRequestGenerator;

pub fn routes(config: &mut ServiceConfig) {
    config
        .route("", post().to(request_authentication))
        .route("/callback", post().to(callback));
}

async fn request_authentication(context: Data<ApplicationContext>, session: Session) -> Result<HttpResponse, PerRequestError> {
    let generator = AuthenticationRequestGenerator::new(&context.config);
    let authentication_request = generator.generate();

    if let Err(_) = session.insert("google-auth-state", &authentication_request.state) {
        return Err(PerRequestError::ServerError)
    }

    let response = HttpResponse::Found()
        .insert_header((header::LOCATION, authentication_request.request_url))
        .finish();
    Ok(response)
}

#[allow(unused_variables)]
async fn callback(context: Data<ApplicationContext>, session: Session) -> Result<HttpResponse, PerRequestError> {
    todo!()
}
