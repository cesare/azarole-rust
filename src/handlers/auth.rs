use std::sync::Arc;

use actix_session::Session;
use actix_web::{
    http::header, web::{get, post, Data, Query, ServiceConfig}, HttpResponse
};
use log::debug;
use serde::Deserialize;
use thiserror::Error;

use crate::{
    context::ApplicationContext,
    errors::PerRequestError,
};

mod access_token_request;
mod authentication_request;
use access_token_request::AccessTokenRequest;
use authentication_request::AuthenticationRequestGenerator;

pub fn routes(config: &mut ServiceConfig) {
    config
        .route("", post().to(request_authentication))
        .route("/callback", get().to(callback));
}

async fn request_authentication(context: Data<ApplicationContext>, session: Session) -> Result<HttpResponse, PerRequestError> {
    let generator = AuthenticationRequestGenerator::new(&context.config);
    let authentication_request = generator.generate();

    session.insert("google-auth-state", &authentication_request.state)?;
    session.insert("google-auth-nonce", &authentication_request.nonce)?;

    let response = HttpResponse::Found()
        .insert_header((header::LOCATION, authentication_request.request_url))
        .finish();
    Ok(response)
}

#[derive(Deserialize)]
struct CallbackParameters {
    code: Option<String>,
    state: Option<String>,
    error: Option<String>,
}

async fn callback(context: Data<ApplicationContext>, session: Session, params: Query<CallbackParameters>) -> Result<HttpResponse, PerRequestError> {
    let parameters = params.into_inner();
    match parameters {
        CallbackParameters { code: Some(code), state: Some(state), error: None } => {
            handle_success(context, session, code, state).await
        },
        _ => {
            handle_failure(context, session, parameters.error).await
        },
    }
}

#[allow(unused_variables)]
async fn handle_success(context: Data<ApplicationContext>, session: Session, code: String, state: String) -> Result<HttpResponse, PerRequestError> {
    debug!("callback: success {}, {}", code, state);

    let saved_nonce = fetch_saved_string(&session, "google-auth-nonce")?;
    let saved_state = fetch_saved_string(&session, "google-auth-state")?;
    if state != saved_state {
        return Err(PerRequestError::Unauthorized)
    }

    let access_token_request = AccessTokenRequest::new(Arc::clone(&context.into_inner()));
    let access_token_response = access_token_request.execute(&code).await?;

    session.clear();
    session.renew();

    let response = HttpResponse::Ok().finish();
    Ok(response)
}

#[allow(unused_variables)]
async fn handle_failure(context: Data<ApplicationContext>, session: Session, error: Option<String>) -> Result<HttpResponse, PerRequestError> {
    debug!("callback: failure {}", error.unwrap_or_default());

    session.remove("google-auth-state");
    session.remove("google-auth-nonce");

    let response = HttpResponse::Unauthorized().finish();
    Ok(response)
}

fn fetch_saved_string(session: &Session, key: &str) -> Result<String, PerRequestError> {
    match session.get::<String>(key)? {
        Some(state) => Ok(state),
        _ => Err(PerRequestError::Unauthorized),
    }
}

#[derive(Debug, Error)]
enum AuthError {
    #[error("request failed")]
    RequestFailed,
}

impl From<reqwest::Error> for AuthError {
    fn from(_value: reqwest::Error) -> Self {
        Self::RequestFailed
    }
}

impl From<AuthError> for PerRequestError {
    fn from(_value: AuthError) -> Self {
        PerRequestError::ServerError
    }
}
