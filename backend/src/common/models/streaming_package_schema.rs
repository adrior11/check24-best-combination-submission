use mongodb::bson::oid;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Validate)]
pub struct StreamingPackage {
    #[serde(rename = "_id")]
    pub id: oid::ObjectId,

    #[validate(range(min = 1))]
    pub streaming_package_id: u8,

    #[validate(length(min = 1))]
    pub name: String,

    pub monthly_price_cents: Option<u16>,

    #[validate(range(min = 0))]
    pub monthly_price_yearly_subscriptions_in_cents: u16,
}
