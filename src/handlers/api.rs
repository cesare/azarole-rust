use actix_web::web::{post, Data, Path, ReqData, ServiceConfig};
use actix_web::{HttpResponse, Result};
use serde_json::json;

use crate::context::ApplicationContext;
use crate::models::user::User;

pub fn routes(config: &mut ServiceConfig) {
    config
        .route("/workplaces/{workplace_id}/clock_ins", post().to(clock_in));
}

async fn clock_in(_context: Data<ApplicationContext>, current_user: ReqData<User>, path: Path<u32>) -> Result<HttpResponse> {
    let workplace_id = path.into_inner();
    let response_json = json!({
        "user_id": current_user.id,
        "workplace_id": workplace_id,
    });
    let response = HttpResponse::Created().json(response_json);
    Ok(response)
}
