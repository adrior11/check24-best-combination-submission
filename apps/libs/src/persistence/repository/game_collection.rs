use crate::models::schemas::Game;
use anyhow::Context;
use futures::stream::TryStreamExt;
use mongodb::{bson, Collection};

pub struct GamesRepository {
    collection: Collection<Game>,
}

impl GamesRepository {
    pub fn new(collection: Collection<Game>) -> Self {
        Self { collection }
    }

    pub async fn fetch_games(&self, teams: &[String]) -> anyhow::Result<Vec<Game>> {
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
        let pipeline = vec![
            bson::doc! {
                "$match": {
                    "$or": [
                        { "team_home": { "$in": teams } },
                        { "team_away": { "$in": teams } }
                    ]
                }
            },
            bson::doc! {
                "$project": {
                    "_id": 0,
                    "game_id": 1
                }
            },
        ];

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
