use super::StreamingPackageDto;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct BestCombinationDto {
    // TODO: Combination types (min packages, min cost, etc.)
    pub services: Vec<StreamingPackageDto>,
    pub total_monthly_price_cents: u32,
    pub total_monthly_price_yearly_subscription_in_cents: u32,
}
