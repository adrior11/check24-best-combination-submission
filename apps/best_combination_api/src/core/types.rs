use async_graphql::{Enum, InputObject, SimpleObject};
use libs::models::dtos::BestCombinationDto;
use serde::{Deserialize, Serialize};

#[derive(Enum, Copy, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum FetchStatus {
    Ready,
    Processing,
    Error,
}

#[derive(SimpleObject, Serialize)]
pub struct FetchResult {
    pub status: FetchStatus,
    pub data: Option<Vec<BestCombinationDto>>,
}

#[derive(InputObject, Deserialize)]
pub struct FetchOptions {
    #[graphql(default = 1)]
    pub limit: usize,
}
