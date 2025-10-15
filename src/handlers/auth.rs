use std::ops::Deref;

use actix_session::Session;
use actix_web::{
    web::{post, Data, Form, ServiceConfig}, HttpResponse
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use thiserror::Error;

use crate::{
    config::ApplicationConfig, context::ApplicationContext, errors::PerRequestError
};
use super::views::UserView;

mod access_token_request;
mod authentication_request;
mod id_token_verifier;
mod user_finder;
use access_token_request::AccessTokenRequest;
use authentication_request::AuthenticationRequestGenerator;
use id_token_verifier::IdTokenVerifier;
use user_finder::UserFinder;

pub(super) fn routes(config: &mut ServiceConfig) {
    config
        .route("", post().to(request_authentication))
        .route("/callback", post().to(callback));
}

async fn request_authentication(context: Data<ApplicationContext>, session: Session) -> Result<HttpResponse, PerRequestError> {
    let generator = AuthenticationRequestGenerator::new(&context.config);
    let authentication_request = generator.generate();

    session.insert("google-auth-state", &authentication_request.state)?;
    session.insert("google-auth-nonce", &authentication_request.nonce)?;

    let response_json = json!({
        "location": authentication_request.request_url,
    });
    let response = HttpResponse::Ok().json(response_json);
    Ok(response)
}

#[derive(Deserialize)]
struct CallbackParameters {
    code: Option<String>,
    state: Option<String>,
    error: Option<String>,
}

async fn callback(context: Data<ApplicationContext>, session: Session, params: Form<CallbackParameters>) -> Result<HttpResponse, PerRequestError> {
    let parameters = params.into_inner();
    match parameters {
        CallbackParameters { code: Some(code), state: Some(state), error: None } => {
            handle_success(context, session, code, state).await
        },
        _ => {
            handle_failure(session).await
        },
    }
}

async fn handle_success(context: Data<ApplicationContext>, session: Session, code: String, state: String) -> Result<HttpResponse, PerRequestError> {
    let saved_nonce = fetch_saved_string(&session, "google-auth-nonce")?;
    let saved_state = fetch_saved_string(&session, "google-auth-state")?;
    if state != saved_state {
        return Err(PerRequestError::Unauthorized)
    }

    let access_token_request = AccessTokenRequest::new(&context);
    let access_token_response = access_token_request.execute(&code).await?;

    let id_token_verifier = IdTokenVerifier::new(&access_token_response.id_token, &saved_nonce);
    let claims = id_token_verifier.verify().await?;

    let finder = UserFinder::new(&context, &claims.sub);
    let user = finder.execute().await?;

    session.clear();
    session.renew();
    session.insert("user_id", user.id)?;

    let response_json = json!({
        "user": UserView::new(&user),
    });
    let response = HttpResponse::Ok().json(response_json);
    Ok(response)
}

async fn handle_failure(session: Session) -> Result<HttpResponse, PerRequestError> {
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

    #[error("invalid id token")]
    InvalidIdToken,
}

impl From<reqwest::Error> for AuthError {
    fn from(_value: reqwest::Error) -> Self {
        Self::RequestFailed
    }
}

impl From<jsonwebtoken::errors::Error> for AuthError {
    fn from(_value: jsonwebtoken::errors::Error) -> Self {
        Self::InvalidIdToken
    }
}

impl From<AuthError> for PerRequestError {
    fn from(_value: AuthError) -> Self {
        PerRequestError::ServerError
    }
}

#[derive(Serialize)]
#[repr(transparent)]
struct RedirectUri(String);

impl RedirectUri {
    fn new(config: &ApplicationConfig) -> Self {
        Self(format!("{}/signin/callback", config.frontend.base_url))
    }
}

impl Deref for RedirectUri {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
