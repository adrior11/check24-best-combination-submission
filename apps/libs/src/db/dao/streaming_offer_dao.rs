use futures::stream::TryStreamExt;
use mongodb::{bson, Collection};

use crate::models::schemas::StreamingOfferSchema;

#[allow(unused)]
pub struct StreamingOfferDao {
    collection: Collection<StreamingOfferSchema>,
}

#[allow(unused)]
impl StreamingOfferDao {
    pub fn new(collection: Collection<StreamingOfferSchema>) -> Self {
        Self { collection }
    }

    pub async fn get_offers_by_game_ids(
        &self,
        game_ids: &[u32],
    ) -> anyhow::Result<Vec<StreamingOfferSchema>> {
        let filter = bson::doc! { "game_id": { "$in": game_ids } };
        let cursor = self.collection.find(filter).await?;
        let offers = cursor.try_collect().await?;
        Ok(offers)
    }

    pub async fn get_offers_by_package_ids(
        &self,
        package_ids: &[u32],
    ) -> anyhow::Result<Vec<StreamingOfferSchema>> {
        let filter = bson::doc! { "streaming_package_id": { "$in": package_ids } };
        let cursor = self.collection.find(filter).await?;
        let offers = cursor.try_collect().await?;
        Ok(offers)
    }
}
