use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};

use super::BestCombinationPackageDto;

#[derive(SimpleObject, Clone, Eq, PartialEq, Serialize, Deserialize, Debug)]
pub struct BestCombinationDto {
    pub packages: Vec<BestCombinationPackageDto>,
    pub combined_monthly_price_cents: usize,
    pub combined_monthly_price_yearly_subscription_in_cents: usize,
    pub combined_coverage: u8,
    pub index: usize,
}

impl BestCombinationDto {
    pub fn new(
        packages: Vec<BestCombinationPackageDto>,
        combined_monthly_price_cents: usize,
        combined_monthly_price_yearly_subscription_in_cents: usize,
        combined_coverage: u8,
        index: usize,
    ) -> Self {
        BestCombinationDto {
            packages,
            combined_monthly_price_cents,
            combined_monthly_price_yearly_subscription_in_cents,
            combined_coverage,
            index,
        }
    }

    /// Compares two BestCombinationDto instances, ignoring the `index` field.
    pub fn is_duplicate_of(&self, other: &Self) -> bool {
        self.packages == other.packages
            && self.combined_monthly_price_cents == other.combined_monthly_price_cents
            && self.combined_monthly_price_yearly_subscription_in_cents
                == other.combined_monthly_price_yearly_subscription_in_cents
            && self.combined_coverage == other.combined_coverage
    }
}
