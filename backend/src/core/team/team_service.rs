use crate::{
    app,
    common::{constants, models::Game},
};
use actix_web::web;
use mongodb::{bson, error};
use std::sync;

pub async fn fetch_all_team_names(
    app_state: web::Data<sync::Arc<app::AppState>>,
) -> Result<mongodb::Cursor<bson::Document>, error::Error> {
    let mongo_client = &app_state.mongo_client;

    let collection: mongodb::Collection<Game> = mongo_client
        .database(constants::DATABASE_NAME)
        .collection(constants::GAME_COLLECTION_NAME);

    let pipeline = build_team_names_pipeline();

    collection.aggregate(pipeline).await.map_err(|e| {
        log::error!("Error executing aggregation pipeline: {:?}", e);
        e
    })
}

fn build_team_names_pipeline() -> Vec<bson::Document> {
    let union_stage = bson::doc! {
        "$project": {
            "teams": {
                "$setUnion": [
                    ["$team_away", "$team_home"]
                ]
            }
        }
    };

    let unwind_stage = bson::doc! {
        "$unwind": "$teams"
    };

    let group_stage = bson::doc! {
        "$group": {
            "_id": "$teams"
        }
    };

    let project_stage = bson::doc! {
        "$project": {
            "_id": 0,
            "team_name": "$_id"
        }
    };

    vec![union_stage, unwind_stage, group_stage, project_stage]
}
