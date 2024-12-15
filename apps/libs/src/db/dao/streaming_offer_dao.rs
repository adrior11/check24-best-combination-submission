use crate::models::schemas::StreamingOfferSchema;
use futures::stream::TryStreamExt;
use mongodb::{bson, Collection};

pub struct StreamingOfferDao {
    collection: Collection<StreamingOfferSchema>,
}

impl StreamingOfferDao {
    pub fn new(collection: Collection<StreamingOfferSchema>) -> Self {
        Self { collection }
    }

    pub async fn fetch_offers(
        &self,
        game_ids: &[u32],
    ) -> anyhow::Result<Vec<StreamingOfferSchema>> {
        let filter = bson::doc! { "game_id": { "$in": game_ids } };
        let cursor = self.collection.find(filter).await?;
        let offers = cursor.try_collect().await?;
        Ok(offers)
    }
}
