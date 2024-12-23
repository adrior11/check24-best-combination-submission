use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};

use crate::models::schemas::StreamingOfferSchema;

#[derive(SimpleObject, Clone, Serialize, Deserialize, Debug)]
pub struct StreamingOfferDto {
    pub game_id: u32,
    pub streaming_package_id: u32,
    pub live: u8,
    pub highlights: u8,
}

impl From<StreamingOfferSchema> for StreamingOfferDto {
    fn from(o: StreamingOfferSchema) -> Self {
        StreamingOfferDto {
            game_id: o.game_id,
            streaming_package_id: o.streaming_package_id,
            live: o.live,
            highlights: o.highlights,
        }
    }
}
