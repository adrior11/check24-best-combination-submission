use crate::models::schemas::StreamingOffer;
use futures::stream::TryStreamExt;
use mongodb::{bson, Collection};

pub struct OffersRepository {
    collection: Collection<StreamingOffer>,
}

impl OffersRepository {
    pub fn new(collection: Collection<StreamingOffer>) -> Self {
        Self { collection }
    }

    pub async fn fetch_offers(&self, game_ids: &[u32]) -> anyhow::Result<Vec<StreamingOffer>> {
        let filter = bson::doc! { "game_id": { "$in": game_ids } };
        let cursor = self.collection.find(filter).await?;
        let offers = cursor.try_collect().await?;
        Ok(offers)
    }
}
