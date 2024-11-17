use crate::{app, libs::metrics};
use actix_web::{http::header, web};
use std::sync;

/// The `/metrics` endpoint for exposing Prometheus metrics.
#[actix_web::get("/metrics")]
pub async fn get_metrics(
    app_state: web::Data<sync::Arc<app::AppState>>,
) -> impl actix_web::Responder {
    let registry = &app_state.registry;

    let metrics = metrics::gather_metrics(registry);

    actix_web::HttpResponse::Ok()
        .content_type(header::ContentType::plaintext())
        .body(metrics)
}
