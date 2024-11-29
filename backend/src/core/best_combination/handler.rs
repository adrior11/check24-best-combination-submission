use super::{dto::BestCombinationRequest, service};
use crate::app::AppState;
use actix_web::{
    self,
    web::{Data, Query},
    HttpResponse,
};
use std::sync::Arc;

#[actix_web::get("/best_combination")]
pub async fn find_best_combination(
    app_state: Data<Arc<AppState>>,
    query: Query<BestCombinationRequest>,
) -> HttpResponse {
    match service::find_best_combination(app_state.clone(), query.into_inner()).await {
        Ok(bundles) => HttpResponse::Ok().json(bundles),
        Err(e) => {
            log::error!("Error in find_best_combination_service: {}", e);
            HttpResponse::InternalServerError().body("Internal Server Error")
        }
    }
}
