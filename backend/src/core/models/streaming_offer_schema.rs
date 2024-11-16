use mongodb::bson::oid;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Validate)]
pub struct StreamingOffer {
    #[serde(rename = "_id")]
    pub id: oid::ObjectId,

    #[validate(range(min = 1))]
    pub game_id: u8,

    #[validate(range(min = 1))]
    pub streaming_package_id: u8,

    #[validate(range(min = 0, max = 1))]
    pub live: u8,

    #[validate(range(min = 0, max = 1))]
    pub highlights: u8,
}
