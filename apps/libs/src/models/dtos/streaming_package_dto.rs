use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};

use crate::models::schemas::StreamingPackageSchema;

#[derive(SimpleObject, Clone, Serialize, Deserialize, Debug)]
pub struct StreamingPackageDto {
    pub streamin_package_id: u32,
    pub name: String,
    pub monthly_price_cents: Option<u16>,
    pub monthly_price_yearly_subscription_in_cents: u16,
}

impl From<StreamingPackageSchema> for StreamingPackageDto {
    fn from(o: StreamingPackageSchema) -> Self {
        StreamingPackageDto {
            streamin_package_id: o.streaming_package_id,
            name: o.name,
            monthly_price_cents: o.monthly_price_cents,
            monthly_price_yearly_subscription_in_cents: o
                .monthly_price_yearly_subscription_in_cents,
        }
    }
}
