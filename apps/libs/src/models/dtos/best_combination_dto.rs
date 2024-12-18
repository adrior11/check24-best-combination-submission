use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};

// TODO: It's important to track packages without monthly subscriptions
#[derive(SimpleObject, Clone, Eq, PartialEq, Serialize, Deserialize, Debug)]
pub struct BestCombinationDto {
    pub packages: Vec<usize>,
    pub combined_monthly_price_cents: usize,
    pub combined_monthly_price_yearly_subscription_in_cents: usize,
    pub coverage: u8,
}
