use std::collections::BTreeSet;

use anyhow::Context;
use futures::TryStreamExt;
use mongodb::{bson, Collection};

use super::documents;
use crate::models::{dtos::BestCombinationSubsetDto, schemas::StreamingPackageSchema};

pub struct StreamingPackageDao {
    collection: Collection<StreamingPackageSchema>,
}

impl StreamingPackageDao {
    pub fn new(collection: Collection<StreamingPackageSchema>) -> Self {
        Self { collection }
    }

    pub async fn find_packages_by_ids(
        &self,
        package_ids: &[u32],
    ) -> anyhow::Result<Vec<StreamingPackageSchema>> {
        let filter = bson::doc! { "streaming_package_id": { "$in": package_ids } };
        let cursor = self.collection.find(filter).await?;
        let packages = cursor.try_collect().await?;
        Ok(packages)
    }

    pub async fn aggregate_subsets_by_game_ids(
        &self,
        game_ids: &BTreeSet<usize>,
    ) -> anyhow::Result<Vec<BestCombinationSubsetDto>> {
        let game_ids: Vec<u32> = game_ids.iter().map(|&x| x as u32).collect();
        let pipeline = documents::preprocess_subsets_pipeline(&game_ids);
        let mut cursor = self
            .collection
            .aggregate(pipeline)
            .await
            .context("Failed to aggregate subsets")?;
        let mut subsets: Vec<BestCombinationSubsetDto> = Vec::new();

        while let Some(doc) = cursor
            .try_next()
            .await
            .context("Failed to get doc from aggregation cursor")?
        {
            let subset: BestCombinationSubsetDto = bson::from_bson(bson::Bson::Document(doc))?;
            subsets.push(subset);
        }

        Ok(subsets)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        constants::{DATABASE_NAME, STREAMING_PACKAGE_COLLECTION_NAME},
        db::{DocumentDatabaseConnector, MongoClient},
    };
    use std::env;

    #[tokio::test]
    async fn test_find_packages_by_ids() {
        dotenv::dotenv().ok();
        let uri = env::var("MONGODB_URI").expect("MONGODB_URI must be set in env");
        let mongo_client = MongoClient::init(&uri, DATABASE_NAME).await;
        let package_dao = StreamingPackageDao::new(
            mongo_client.get_collection(STREAMING_PACKAGE_COLLECTION_NAME),
        );

        let package_ids = vec![
            37, 55, 14, 10, 38, 17, 13, 19, 15, 2, 56, 54, 43, 18, 20, 50, 47, 35, 4, 41, 39, 53,
            52, 16, 44, 49, 3, 36, 40,
        ];
        let packages = package_dao
            .find_packages_by_ids(&package_ids)
            .await
            .unwrap();

        assert!(!packages.is_empty());
        assert!(packages.len() == package_ids.len());
        for package in packages {
            assert!(package_ids.contains(&package.streaming_package_id));
        }
    }

    #[tokio::test]
    async fn test_aggregate_subsets_by_game_ids() {
        dotenv::dotenv().ok();
        let uri = env::var("MONGODB_URI").expect("MONGODB_URI must be set in env");
        let mongo_client = MongoClient::init(&uri, DATABASE_NAME).await;
        let package_dao = StreamingPackageDao::new(
            mongo_client.get_collection(STREAMING_PACKAGE_COLLECTION_NAME),
        );

        let game_ids = BTreeSet::from([
            52, 69, 76, 79, 103, 89, 113, 121, 125, 139, 146, 151, 161, 171, 186, 193, 196, 212,
            214, 219, 225, 240, 251, 257, 261, 272, 284, 293, 307, 320, 302, 325, 337, 349, 356,
            5305, 5320, 5325, 5330, 5341, 5349, 5364, 5367, 5383, 5386, 5394, 5404, 5416, 5436,
            5440, 5422, 5449, 5459, 5467, 5474, 5483, 5492, 5501, 5511, 5525, 5529, 5541, 5548,
            5557, 5566, 5584, 5573, 5593, 7354, 7890, 8440, 8466, 8486, 8514, 8503, 8533, 8568,
            8560, 8845,
        ]);
        let subsets = package_dao
            .aggregate_subsets_by_game_ids(&game_ids)
            .await
            .unwrap();

        assert!(!subsets.is_empty());
        for subset in subsets {
            for element in subset.elements {
                assert!(game_ids.contains(&element.game_id))
            }
        }
    }
}
