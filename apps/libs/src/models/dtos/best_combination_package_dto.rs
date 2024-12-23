use std::collections::HashMap;

use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};

#[derive(SimpleObject, Clone, Eq, PartialEq, Serialize, Deserialize, Debug)]
pub struct BestCombinationPackageDto {
    pub id: usize,
    pub coverage: HashMap<String, (u8, u8)>,
    pub monthly_price_cents: Option<usize>,
    pub monthly_price_yearly_subscription_in_cents: usize,
}

impl BestCombinationPackageDto {
    pub fn new(
        id: usize,
        coverage: Vec<(&str, (u8, u8))>,
        monthly_price_cents: Option<usize>,
        monthly_price_yearly_subscription_in_cents: usize,
    ) -> Self {
        BestCombinationPackageDto {
            id,
            coverage: coverage
                .into_iter()
                .map(|(k, v)| (k.to_string(), v))
                .collect(),
            monthly_price_cents,
            monthly_price_yearly_subscription_in_cents,
        }
    }
}
