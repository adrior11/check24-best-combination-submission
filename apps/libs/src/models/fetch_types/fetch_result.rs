use async_graphql::SimpleObject;
use serde::Serialize;

use super::FetchStatus;
use crate::models::dtos::BestCombinationDto;

#[derive(SimpleObject, Serialize)]
pub struct FetchResult {
    pub status: FetchStatus,
    pub ids: Vec<usize>,
    pub data: Option<Vec<BestCombinationDto>>,
}
