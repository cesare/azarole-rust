use std::rc::Rc;

use actix_session::SessionExt;
use actix_web::{Error, HttpMessage};
use actix_web::body::MessageBody;
use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::web::Data;
use futures_util::future::{ok, LocalBoxFuture, Ready};

use crate::context::ApplicationContext;
use crate::models::{User, UserId};

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
        let service = Rc::clone(&self.service);

        Box::pin(async move {
            let session = req.get_session();
            let value = session.get::<UserId>("user_id")?;
            if value.is_none() {
                return Err(actix_web::error::ErrorUnauthorized("unauthorized"));
            }

            let user_id = value.unwrap();

            let context: &Data<ApplicationContext> = req.app_data().unwrap();
            let result: Result<Option<User>, sqlx::error::Error> =
                sqlx::query_as("select id from users where id = $1")
                    .bind(&user_id)
                    .fetch_optional(&context.database.pool)
                    .await;

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
