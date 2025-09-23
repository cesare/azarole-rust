use actix_web::web::{post, Data, Path, ServiceConfig};
use actix_web::{HttpResponse, Result};
use serde_json::json;

use crate::context::ApplicationContext;

pub fn routes(config: &mut ServiceConfig) {
    config
        .route("/workplaces/{workplace_id}/clock_ins", post().to(clock_in));
}

async fn clock_in(_context: Data<ApplicationContext>, path: Path<u32>) -> Result<HttpResponse> {
    let workplace_id = path.into_inner();
    let response_json = json!({
        "workplace_id": workplace_id,
    });
    let response = HttpResponse::Created().json(response_json);
    Ok(response)
}
