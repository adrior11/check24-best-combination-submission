use crate::models::schemas::GameSchema;
use anyhow::Context;
use futures::stream::TryStreamExt;
use mongodb::{bson, Collection};

use super::pipelines;

pub struct GameDao {
    collection: Collection<GameSchema>,
}

impl GameDao {
    pub fn new(collection: Collection<GameSchema>) -> Self {
        Self { collection }
    }

    pub async fn fetch_games(&self, teams: &[String]) -> anyhow::Result<Vec<GameSchema>> {
        let filter = bson::doc! {
            "$or": [
                { "team_home": { "$in": teams } },
                { "team_away": { "$in": teams } },
            ]
        };

        let cursor = self.collection.find(filter).await?;
        let games = cursor.try_collect().await?;

        Ok(games)
    }

    pub async fn fetch_game_ids(&self, teams: &[String]) -> anyhow::Result<Vec<u32>> {
        let pipeline = pipelines::games_by_teams_pipeline(teams);
        let mut cursor = self.collection.aggregate(pipeline).await?;
        let mut game_ids = Vec::new();

        while let Some(doc) = cursor.try_next().await? {
            if let Ok(game_id) = bson::from_bson::<u32>(
                doc.get("game_id")
                    .context("Failed to parse game_id")?
                    .clone(),
            ) {
                game_ids.push(game_id);
            }
        }

        Ok(game_ids)
    }
}
