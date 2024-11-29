use serde::{Deserialize, Serialize};

use crate::common::models::StreamingOffer;

#[derive(Deserialize)]
pub struct BestCombinationRequest {
    pub teams: Vec<String>,
    #[allow(dead_code)] // TODO: Replaced with GraphQL
    pub limit: Option<usize>,
}

// TODO: Add features
#[derive(Deserialize, Serialize)]
pub struct StreamingBundle {
    pub offers: Vec<StreamingOffer>,
    pub offer_count: usize,
    pub combined_monthly_price_cents: u8,
    pub combined_monthly_price_yearly_subscriptions_in_cents: u8,
}
