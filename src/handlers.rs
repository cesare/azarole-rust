use actix_web::web::{ServiceConfig, scope};

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

pub(super) fn routes(config: &mut ServiceConfig) {
    config
        .service(scope("/api").wrap(RequireApiKey::new()).configure(api::routes))
        .service(scope("/auth/google").configure(auth::routes))
        .service(scope("/signout").configure(signout::routes))
        .service(scope("").wrap(RequireSignin::new()).configure(backend_routes));
}

fn backend_routes(config: &mut ServiceConfig) {
    config
        .service(scope("/api_keys").configure(api_keys::routes))
        .service(scope("/current_user").configure(current_user::routes))
        .service(scope("/workplaces/{workplace_id}/attendance_records").configure(attendance_records::routes))
        .service(scope("/workplaces").configure(workplaces::routes));
}
