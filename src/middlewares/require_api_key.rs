use std::rc::Rc;

use actix_web::body::MessageBody;
use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::web::Data;
use actix_web::{Error, FromRequest, HttpMessage};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use futures_util::future::{LocalBoxFuture, Ready, ok};

mod api_key_authenticator;

use crate::context::ApplicationContext;
use api_key_authenticator::ApiKeyAuthenticator;

pub struct RequireApiKey;

impl RequireApiKey {
    pub fn new() -> Self {
        Self {}
    }
}

impl<S, B> Transform<S, ServiceRequest> for RequireApiKey
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    S::Error: 'static,
    B: MessageBody + 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = RequireApiKeyMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(RequireApiKeyMiddleware {
            service: Rc::new(service),
        })
    }
}

pub struct RequireApiKeyMiddleware<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for RequireApiKeyMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: MessageBody + 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    actix_service::forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let service = Rc::clone(&self.service);

        Box::pin(async move {
            let context: &Data<ApplicationContext> = req.app_data().unwrap();

            let bearer_auth = BearerAuth::extract(req.request()).await?;
            let token = bearer_auth.token();

            let authenticator = ApiKeyAuthenticator::new(context, token);
            let result = authenticator.authenticate().await;
            match result {
                Ok(Some(user)) => {
                    req.extensions_mut().insert(user);
                    let response = service.call(req).await?;
                    Ok(response)
                }
                Ok(None) => Err(actix_web::error::ErrorUnauthorized("unauthorized")),
                Err(_err) => Err(actix_web::error::ErrorInternalServerError(
                    "internal server error",
                )),
            }
        })
    }
}
