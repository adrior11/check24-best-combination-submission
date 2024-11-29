use super::set_cover;
use crate::{
    app::AppState,
    common::{
        constants::{
            DATABASE_NAME, GAME_COLLECTION_NAME, STREAMING_OFFER_COLLECTION_NAME,
            STREAMING_PACKAGE_COLLECTION_NAME,
        },
        models::{Game, StreamingOffer, StreamingPackage},
    },
};
use actix_web::web::Data;
use futures::TryStreamExt;
use mongodb::{bson, error, Collection};
use std::{collections::HashSet, sync::Arc};

pub async fn find_best_combination(
    app_state: Data<Arc<AppState>>,
    teams: Vec<String>,
) -> anyhow::Result<HashSet<StreamingPackage>> {
    log::info!("Invoking find_best_combination with teams: {:?}", teams);

    let mongo_client = &app_state.mongo_client;
    let _redis_client = &app_state.redis_client;

    let games = fetch_games(mongo_client, &teams).await?;
    let game_ids: Vec<u32> = games.iter().map(|g| g.game_id).collect();
    log::debug!("Fetched games: {:?}", games);

    let offers = fetch_offers(mongo_client, &game_ids).await?;
    let package_ids: Vec<u32> = offers.iter().map(|o| o.streaming_package_id).collect();
    log::debug!("Fetched offers: {:?}", offers);

    let packages = fetch_packages(mongo_client, &package_ids).await?;
    log::debug!("Fetched packages: {:?}", packages);

    log::info!(
        "Calculating best combination for {} games, {} offers, and {} packages.",
        game_ids.len(),
        offers.len(),
        packages.len()
    );

    let result = set_cover::set_cover_best_combination(&game_ids, &packages, &offers)?;

    log::info!(
        "Best combination calculation complete. Selected {} packages.",
        result.len()
    );

    Ok(result)
}

async fn fetch_games(mongo_client: &mongodb::Client, teams: &[String]) -> error::Result<Vec<Game>> {
    let games_collection: mongodb::Collection<Game> = mongo_client
        .database(DATABASE_NAME)
        .collection(GAME_COLLECTION_NAME);

    let filter = bson::doc! {
        "$or": [
            { "team_home": { "$in": teams } },
            { "team_away": { "$in": teams } },
        ]
    };

    let cursor = games_collection.find(filter).await?;
    let games: Vec<Game> = cursor.try_collect().await?;

    Ok(games)
}

async fn fetch_offers(
    mongo_client: &mongodb::Client,
    game_ids: &[u32],
) -> error::Result<Vec<StreamingOffer>> {
    let offers_collection: Collection<StreamingOffer> = mongo_client
        .database(DATABASE_NAME)
        .collection(STREAMING_OFFER_COLLECTION_NAME);

    let filter = bson::doc! { "game_id": { "$in": game_ids } };
    let cursor = offers_collection.find(filter).await?;
    let offers: Vec<StreamingOffer> = cursor.try_collect().await?;

    Ok(offers)
}

async fn fetch_packages(
    mongo_client: &mongodb::Client,
    package_ids: &[u32],
) -> error::Result<Vec<StreamingPackage>> {
    let packages_collection: Collection<StreamingPackage> = mongo_client
        .database(DATABASE_NAME)
        .collection(STREAMING_PACKAGE_COLLECTION_NAME);

    let filter = bson::doc! { "streaming_package_id": { "$in": package_ids } };
    let cursor = packages_collection.find(filter).await?;
    let packages: Vec<StreamingPackage> = cursor.try_collect().await?;

    Ok(packages)
}
