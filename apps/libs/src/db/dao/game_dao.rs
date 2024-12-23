use anyhow::Context;
use futures::stream::TryStreamExt;
use mongodb::{bson, Collection};

use super::{documents, util};
use crate::models::schemas::GameSchema;

pub struct GameDao {
    collection: Collection<GameSchema>,
}

impl GameDao {
    pub fn new(collection: Collection<GameSchema>) -> Self {
        Self { collection }
    }

    pub async fn get_teams(&self) -> anyhow::Result<Vec<String>> {
        let pipeline = documents::aggregate_teams_pipeline();
        let mut cursor = self.collection.aggregate(pipeline).await?;

        if let Some(doc) = cursor.try_next().await? {
            if let Ok(teams) = bson::from_bson::<Vec<String>>(
                doc.get("teams").context("Failed to parse teams")?.clone(),
            ) {
                return Ok(teams);
            }
        }

        Ok(Vec::new())
    }

    pub async fn get_tournaments(&self) -> anyhow::Result<Vec<String>> {
        let pipeline = documents::aggregate_tournaments_pipeline();
        let mut cursor = self.collection.aggregate(pipeline).await?;

        if let Some(doc) = cursor.try_next().await? {
            if let Ok(tournaments) = bson::from_bson::<Vec<String>>(
                doc.get("tournaments")
                    .context("Failed to parse tournaments")?
                    .clone(),
            ) {
                return Ok(tournaments);
            }
        }

        Ok(Vec::new())
    }

    pub async fn find_games_by_teams(&self, teams: &[String]) -> anyhow::Result<Vec<GameSchema>> {
        let filter = documents::filter_teams(teams);
        let cursor = self.collection.find(filter).await?;
        let games = cursor.try_collect().await?;
        Ok(games)
    }

    pub async fn find_games_by_tournaments(
        &self,
        tournaments: &[String],
    ) -> anyhow::Result<Vec<GameSchema>> {
        let filter = documents::filter_tournaments(tournaments);
        let cursor = self.collection.find(filter).await?;
        let games = cursor.try_collect().await?;
        Ok(games)
    }

    pub async fn aggregate_game_ids(
        &self,
        teams: Option<Vec<String>>,
        tournaments: Option<Vec<String>>,
    ) -> anyhow::Result<Vec<usize>> {
        let match_query = util::build_match_query(teams, tournaments);

        // Return an empty array, if no input has been provided
        if match_query.is_none() {
            return Ok(vec![]);
        }

        let pipeline = vec![
            bson::doc! { "$match": match_query.unwrap() },
            documents::project_game_id(),
        ];

        let mut cursor = self.collection.aggregate(pipeline).await?;
        let mut game_ids = Vec::new();

        while let Some(doc) = cursor.try_next().await? {
            if let Ok(game_id) = bson::from_bson::<usize>(
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        constants::{DATABASE_NAME, GAME_COLLECTION_NAME},
        db::{DocumentDatabaseConnector, MongoClient},
    };
    use std::env;

    #[tokio::test]
    async fn test_get_games() {
        dotenv::dotenv().ok();
        let uri = env::var("MONGODB_URI").expect("MONGODB_URI must be set in env");
        let mongo_client = MongoClient::init(&uri, DATABASE_NAME).await;
        let game_dao = GameDao::new(mongo_client.get_collection(GAME_COLLECTION_NAME));

        let games = game_dao.get_teams().await.unwrap();

        assert!(!games.is_empty());
    }

    #[tokio::test]
    async fn test_get_tournaments() {
        dotenv::dotenv().ok();
        let uri = env::var("MONGODB_URI").expect("MONGODB_URI must be set in env");
        let mongo_client = MongoClient::init(&uri, DATABASE_NAME).await;
        let game_dao = GameDao::new(mongo_client.get_collection(GAME_COLLECTION_NAME));

        let tournaments = game_dao.get_tournaments().await.unwrap();

        assert!(!tournaments.is_empty());
        assert!(tournaments.len() == 43);
    }

    #[tokio::test]
    async fn test_find_games_by_teams() {
        dotenv::dotenv().ok();
        let uri = env::var("MONGODB_URI").expect("MONGODB_URI must be set in env");
        let mongo_client = MongoClient::init(&uri, DATABASE_NAME).await;
        let game_dao = GameDao::new(mongo_client.get_collection(GAME_COLLECTION_NAME));

        let teams = vec!["Bayern München".to_string()];
        let games = game_dao.find_games_by_teams(&teams).await.unwrap();

        assert!(!games.is_empty());
        assert!(games.len() == 79);
    }

    #[tokio::test]
    async fn test_find_games_by_tournaments() {
        dotenv::dotenv().ok();
        let uri = env::var("MONGODB_URI").expect("MONGODB_URI must be set in env");
        let mongo_client = MongoClient::init(&uri, DATABASE_NAME).await;
        let game_dao = GameDao::new(mongo_client.get_collection(GAME_COLLECTION_NAME));

        let tournaments = vec!["Europameisterschaft 2024".to_string()];
        let games = game_dao
            .find_games_by_tournaments(&tournaments)
            .await
            .unwrap();

        assert!(!games.is_empty());
        assert!(games.len() == 51);
    }

    #[tokio::test]
    async fn test_aggregate_game_ids_without_input() {
        dotenv::dotenv().ok();
        let uri = env::var("MONGODB_URI").expect("MONGODB_URI must be set in env");
        let mongo_client = MongoClient::init(&uri, DATABASE_NAME).await;
        let game_dao = GameDao::new(mongo_client.get_collection(GAME_COLLECTION_NAME));

        let game_ids = game_dao.aggregate_game_ids(None, None).await.unwrap();
        assert!(game_ids.is_empty());
    }

    #[tokio::test]
    async fn test_aggregate_game_ids_by_teams() {
        dotenv::dotenv().ok();
        let uri = env::var("MONGODB_URI").expect("MONGODB_URI must be set in env");
        let mongo_client = MongoClient::init(&uri, DATABASE_NAME).await;
        let game_dao = GameDao::new(mongo_client.get_collection(GAME_COLLECTION_NAME));

        let teams = vec!["Bayern München".to_string()];
        let mut game_ids = game_dao
            .aggregate_game_ids(Some(teams), None)
            .await
            .unwrap();
        let mut expected = vec![
            52, 69, 76, 79, 103, 89, 113, 121, 125, 139, 146, 151, 161, 171, 186, 193, 196, 212,
            214, 219, 225, 240, 251, 257, 261, 272, 284, 293, 307, 320, 302, 325, 337, 349, 356,
            5305, 5320, 5325, 5330, 5341, 5349, 5364, 5367, 5383, 5386, 5394, 5404, 5416, 5436,
            5440, 5422, 5449, 5459, 5467, 5474, 5483, 5492, 5501, 5511, 5525, 5529, 5541, 5548,
            5557, 5566, 5584, 5573, 5593, 7354, 7890, 8440, 8466, 8486, 8514, 8503, 8533, 8568,
            8560, 8845,
        ];

        game_ids.sort();
        expected.sort();

        assert!(!game_ids.is_empty());
        assert_eq!(game_ids.len(), expected.len());
        assert_eq!(game_ids, expected);
    }

    #[tokio::test]
    async fn test_aggregate_game_ids_by_tournaments() {
        dotenv::dotenv().ok();
        let uri = env::var("MONGODB_URI").expect("MONGODB_URI must be set in env");
        let mongo_client = MongoClient::init(&uri, DATABASE_NAME).await;
        let game_dao = GameDao::new(mongo_client.get_collection(GAME_COLLECTION_NAME));

        let tournaments = vec!["Europameisterschaft 2024".to_string()];
        let mut game_ids = game_dao
            .aggregate_game_ids(None, Some(tournaments))
            .await
            .unwrap();
        let mut expected = vec![
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
            25, 26, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 27, 48,
            49, 50, 51, 29, 28,
        ];

        game_ids.sort();
        expected.sort();

        assert!(!game_ids.is_empty());
        assert_eq!(game_ids.len(), expected.len());
        assert_eq!(game_ids, expected);
    }

    #[tokio::test]
    async fn test_aggregate_game_ids_by_teams_and_tournaments() {
        dotenv::dotenv().ok();
        let uri = env::var("MONGODB_URI").expect("MONGODB_URI must be set in env");
        let mongo_client = MongoClient::init(&uri, DATABASE_NAME).await;
        let game_dao = GameDao::new(mongo_client.get_collection(GAME_COLLECTION_NAME));

        let teams = vec!["Bayern München".to_string()];
        let tournaments = vec!["Europameisterschaft 2024".to_string()];
        let mut game_ids = game_dao
            .aggregate_game_ids(Some(teams), Some(tournaments))
            .await
            .unwrap();
        let mut expected = vec![
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
            25, 26, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 27, 48,
            49, 50, 51, 52, 29, 28, 76, 79, 69, 89, 103, 113, 125, 121, 139, 146, 151, 161, 171,
            186, 193, 196, 212, 214, 219, 225, 240, 257, 261, 251, 272, 284, 293, 302, 307, 320,
            325, 337, 349, 356, 5305, 5320, 5325, 5330, 5341, 5349, 5364, 5367, 5386, 5394, 5383,
            5404, 5416, 5422, 5436, 5449, 5440, 5459, 5467, 5474, 5483, 5492, 5501, 5511, 5525,
            5529, 5541, 5548, 5557, 5566, 5573, 5584, 5593, 7354, 7890, 8440, 8466, 8486, 8503,
            8514, 8533, 8560, 8568, 8845,
        ];

        game_ids.sort();
        expected.sort();

        assert!(!game_ids.is_empty());
        assert_eq!(game_ids.len(), expected.len());
        assert_eq!(game_ids, expected);
    }
}
