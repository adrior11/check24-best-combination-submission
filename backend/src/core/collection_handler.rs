use super::collection_service;
use crate::app;
use actix_web::{http::header, web};
use std::sync;

#[actix_web::get("/collections")]
async fn get_all_collection_names(
    app_state: web::Data<sync::Arc<app::AppState>>,
) -> impl actix_web::Responder {
    match collection_service::fetch_all_collection_names(app_state).await {
        Ok(collections) => actix_web::HttpResponse::Ok()
            .content_type(header::ContentType::json())
            .json(collections),
        Err(e) => {
            log::error!("Error fetching collection names: {:?}", e);
            actix_web::HttpResponse::InternalServerError()
                .content_type(header::ContentType::plaintext())
                .body("Error fetching collection names")
        }
    }
}
