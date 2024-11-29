use crate::{
    core,
    libs::{
        caching, logging,
        metrics::{self, middleware::MetricsMiddleware},
        mongo,
    },
};
use actix_web::{web::Data, App, HttpServer};
use prometheus::Registry;
use std::{io, sync::Arc};

#[allow(dead_code)]
pub struct AppState {
    pub mongo_client: Arc<mongodb::Client>,
    pub redis_client: Arc<redis::Client>,
    pub registry: Arc<Registry>,
}

pub async fn run() -> io::Result<()> {
    dotenv::dotenv().ok();

    logging::init_logging();

    let mongo_client = mongo::init_mongodb().await.unwrap();

    let redis_client = caching::init_redis().await.unwrap();

    let registry = metrics::init_metrics();

    let app_state = Arc::new(AppState {
        mongo_client: Arc::new(mongo_client),
        redis_client: Arc::new(redis_client),
        registry: Arc::new(registry),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(app_state.clone()))
            .wrap(MetricsMiddleware)
            .wrap(logging::request_logger())
            .configure(core::configure_routes)
    })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}
