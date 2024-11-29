use crate::common::models::StreamingPackage;
use async_graphql::SimpleObject;

#[derive(SimpleObject)]
pub struct StreamingPackageGQL {
    pub id: String,
    pub name: String,
    pub streaming_package_id: u32,
    pub monthly_price_cents: Option<u16>,
    pub monthly_price_yearly_subscription_in_cents: u16,
}

impl From<StreamingPackage> for StreamingPackageGQL {
    fn from(package: StreamingPackage) -> Self {
        StreamingPackageGQL {
            id: package.id.to_hex(),
            name: package.name,
            streaming_package_id: package.streaming_package_id,
            monthly_price_cents: package.monthly_price_cents,
            monthly_price_yearly_subscription_in_cents: package
                .monthly_price_yearly_subscription_in_cents,
        }
    }
}
