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

impl BestCombinationDto {
    pub fn new(
        packages: Vec<BestCombinationPackageDto>,
        combined_monthly_price_cents: usize,
        combined_monthly_price_yearly_subscription_in_cents: usize,
        combined_coverage: u8,
    ) -> Self {
        BestCombinationDto {
            packages,
            combined_monthly_price_cents,
            combined_monthly_price_yearly_subscription_in_cents,
            combined_coverage,
        }
    }
}
