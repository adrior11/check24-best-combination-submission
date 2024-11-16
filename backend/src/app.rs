use crate::{
    config::{cache, database, logger},
    core,
};
use actix_web::web;
use std::{io, sync};

#[allow(dead_code)]
pub struct AppState {
    pub mongo_client: sync::Arc<mongodb::Client>,
    pub redis_client: sync::Arc<redis::Client>,
}

pub async fn run() -> io::Result<()> {
    let mongo_client = database::init_mongodb().await.unwrap();

    let redis_client = cache::init_redis().await.unwrap();

    let app_state = sync::Arc::new(AppState {
        mongo_client: sync::Arc::new(mongo_client),
        redis_client: sync::Arc::new(redis_client)
    });

    logger::init_logging();

    actix_web::HttpServer::new(move || {
        actix_web::App::new()
            .app_data(web::Data::new(app_state.clone()))
            .wrap(logger::request_logger())
            .configure(core::configure_routes)
    })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}
