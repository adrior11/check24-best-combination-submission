use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};

use super::BestCombinationPackageDto;

#[derive(SimpleObject, Clone, Eq, PartialEq, Serialize, Deserialize, Debug)]
pub struct BestCombinationDto {
    pub packages: Vec<BestCombinationPackageDto>,
    pub combined_monthly_price_cents: usize,
    pub combined_monthly_price_yearly_subscription_in_cents: usize,
    pub combined_coverage: u8,
}
