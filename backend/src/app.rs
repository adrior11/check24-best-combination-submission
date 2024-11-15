use crate::{
    config::{database, logger},
    core,
};
use actix_web::web;
use std::{io, sync};

#[allow(dead_code)]
pub struct AppState {
    pub client: sync::Arc<mongodb::Client>,
}

pub async fn run() -> io::Result<()> {
    let client = database::init_mongodb().await.unwrap();

    let app_state = sync::Arc::new(AppState {
        client: sync::Arc::new(client),
    });

    logger::init_logging();

    actix_web::HttpServer::new(move || {
        actix_web::App::new()
            .app_data(web::Data::new(app_state.clone()))
            .wrap(logger::request_logger())
            .configure(core::configure_routes)
    })
    .bind(("0.0.0.0", 8000))? // NOTE: Use from .env
    .run()
    .await
}
