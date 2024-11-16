use crate::{app, common::constants};
use actix_web::web;
use mongodb::error;
use std::sync;

pub async fn fetch_collection_names(
    app_state: web::Data<sync::Arc<app::AppState>>,
) -> Result<Vec<String>, error::Error> {
    let mongo_client = &app_state.mongo_client;

    mongo_client
        .database(constants::DATABASE_NAME)
        .list_collection_names()
        .await
}
