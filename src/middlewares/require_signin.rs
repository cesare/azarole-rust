use std::rc::Rc;

use actix_session::SessionExt;
use actix_web::body::MessageBody;
use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::web::Data;
use actix_web::{Error, HttpMessage};
use futures_util::future::{LocalBoxFuture, Ready, ok};

use crate::context::ApplicationContext;
use crate::errors::DatabaseError;
use crate::models::{User, UserId};
use crate::repositories::RepositoryFactory;

pub struct RequireSignin;

impl RequireSignin {
    pub fn new() -> Self {
        Self {}
    }
}

impl<S, B> Transform<S, ServiceRequest> for RequireSignin
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    S::Error: 'static,
    B: MessageBody + 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = RequireSigninMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(RequireSigninMiddleware {
            service: Rc::new(service),
        })
    }
}

pub struct RequireSigninMiddleware<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for RequireSigninMiddleware<S>
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
        async fn find_user(
            request: &ServiceRequest,
            user_id: UserId,
        ) -> Result<Option<User>, DatabaseError> {
            let context: &Data<ApplicationContext> = request.app_data().unwrap();
            let repository = context.repositories.user();
            repository.find_optional(user_id).await
        }

        let service = Rc::clone(&self.service);

        Box::pin(async move {
            let session = req.get_session();
            let value = session
                .get::<UserId>("user_id")
                .inspect_err(|e| log::error!("Failed to fetch user_id from session: {:?}", e))?;
            if value.is_none() {
                return Err(actix_web::error::ErrorUnauthorized("unauthorized"));
            }

            let user_id = value.unwrap();
            let result: Result<Option<User>, DatabaseError> = find_user(&req, user_id).await;

            match result {
                Ok(Some(user)) => {
                    req.extensions_mut().insert(user);
                    service.call(req).await
                }
                Ok(None) => {
                    session.remove("user_id");
                    Err(actix_web::error::ErrorUnauthorized("unauthorized"))
                }
                Err(error) => {
                    log::error!("Failed to fetch user: {:?}", error);
                    Err(actix_web::error::ErrorInternalServerError(
                        "internal server error",
                    ))
                }
            }
        })
    }
}
