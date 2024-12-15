use crate::models::{dtos::SubsetDto, schemas::StreamingPackageSchema};
use futures::TryStreamExt;
use mongodb::{bson, Collection};

use super::pipelines;

pub struct StreamingPackageDao {
    collection: Collection<StreamingPackageSchema>,
}

impl StreamingPackageDao {
    pub fn new(collection: Collection<StreamingPackageSchema>) -> Self {
        Self { collection }
    }

    pub async fn fetch_packages(
        &self,
        package_ids: &[u32],
    ) -> anyhow::Result<Vec<StreamingPackageSchema>> {
        let filter = bson::doc! { "streaming_package_id": { "$in": package_ids } };
        let cursor = self.collection.find(filter).await?;
        let packages = cursor.try_collect().await?;
        Ok(packages)
    }

    pub async fn preprocess_subsets(&self, game_ids: &[u32]) -> anyhow::Result<Vec<SubsetDto>> {
        let pipeline = pipelines::preprocess_subsets_pipeline(game_ids);
        let mut cursor = self.collection.aggregate(pipeline).await?;
        let mut subsets: Vec<SubsetDto> = Vec::new();

        while let Some(doc) = cursor.try_next().await? {
            let subset: SubsetDto = bson::from_bson(bson::Bson::Document(doc))?;
            subsets.push(subset);
        }

        Ok(subsets)
    }
}
