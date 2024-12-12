use crate::models::schemas::StreamingPackage;
use futures::TryStreamExt;
use mongodb::{bson, Collection};

pub struct PackagesRepository {
    collection: Collection<StreamingPackage>,
}

impl PackagesRepository {
    pub fn new(collection: Collection<StreamingPackage>) -> Self {
        Self { collection }
    }

    pub async fn fetch_packages(
        &self,
        package_ids: &[u32],
    ) -> anyhow::Result<Vec<StreamingPackage>> {
        let filter = bson::doc! { "streaming_package_id": { "$in": package_ids } };
        let cursor = self.collection.find(filter).await?;
        let packages = cursor.try_collect().await?;
        Ok(packages)
    }
}
