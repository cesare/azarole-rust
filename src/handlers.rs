use actix_web::{web::{get, resource, scope, ServiceConfig}, HttpResponse};

use crate::middlewares::{
    require_api_key::RequireApiKey,
    require_signin::RequireSignin,
};

mod api;
mod api_keys;
mod attendance_records;
mod auth;
mod current_user;
mod workplaces;
mod signout;
mod views;

pub fn routes(config: &mut ServiceConfig) {
    config
        .service(scope("/api").wrap(RequireApiKey::new()).configure(api::routes))
        .service(scope("/auth/google").configure(auth::routes))
        .service(scope("/signout").configure(signout::routes))
        .service(resource("/ping").route(get().to(HttpResponse::NoContent)))
        .service(scope("").wrap(RequireSignin::new()).configure(backend_routes));
}

fn backend_routes(config: &mut ServiceConfig) {
    config
        .service(scope("/api_keys").configure(api_keys::routes))
        .service(scope("/current_user").configure(current_user::routes))
        .service(scope("/workplaces/{workplace_id}/attendance_records").configure(attendance_records::routes))
        .service(scope("/workplaces").configure(workplaces::routes));
}
