use crate::{
    core,
    libs::{logger, mongo, cache, metrics::{self, middleware}},
};
use actix_web::web;
use std::{io, sync};

#[allow(dead_code)]
pub struct AppState {
    pub mongo_client: sync::Arc<mongodb::Client>,
    pub redis_client: sync::Arc<redis::Client>,
    pub registry: sync::Arc<prometheus::Registry>,
}

pub async fn run() -> io::Result<()> {
    dotenv::dotenv().ok();

    logger::init_logging();

    let mongo_client = mongo::init_mongodb().await.unwrap();

    let redis_client = cache::init_redis().await.unwrap();

    let registry = metrics::init_metrics();

    let app_state = sync::Arc::new(AppState {
        mongo_client: sync::Arc::new(mongo_client),
        redis_client: sync::Arc::new(redis_client),
        registry: sync::Arc::new(registry),
    });

    actix_web::HttpServer::new(move || {
        actix_web::App::new()
            .app_data(web::Data::new(app_state.clone()))
            .wrap(middleware::MetricsMiddleware)
            .wrap(logger::request_logger())
            .configure(core::configure_routes)
    })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}
