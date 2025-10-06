use actix_web::web::{ServiceConfig, scope};

use crate::middlewares::{
    require_api_key::RequireApiKey,
    require_signin::RequireSignin,
};

mod api;
mod auth;
mod workplaces;
mod signout;

pub(super) fn routes(config: &mut ServiceConfig) {
    config
        .service(scope("/api").wrap(RequireApiKey::new()).configure(api::routes))
        .service(scope("/auth/google").configure(auth::routes))
        .service(scope("/signout").configure(signout::routes))
        .service(scope("").wrap(RequireSignin::new()).configure(backend_routes));
}

fn backend_routes(config: &mut ServiceConfig) {
    config
        .service(scope("/workplaces").configure(workplaces::routes));
}
