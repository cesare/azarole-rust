use std::rc::Rc;

use actix_web::{Error, HttpMessage};
use actix_web::body::MessageBody;
use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::web::Data;
use futures_util::future::{ok, LocalBoxFuture, Ready};

use crate::context::ApplicationContext;
use crate::models::api_key_authenticator::ApiKeyAuthenticator;

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
            let authenticator = ApiKeyAuthenticator::new("dummy-token");
            let result = authenticator.authenticate(context.get_ref()).await;
            match result {
                Ok(Some(user)) => {
                    req.extensions_mut().insert(user);
                    let response = service.call(req).await?;
                    Ok(response)
                },
                Ok(None) => {
                    Err(actix_web::error::ErrorUnauthorized("unauthorized"))
                },
                Err(_err) => {
                    Err(actix_web::error::ErrorInternalServerError("internal server error"))
                }
            }
        })
    }
}
