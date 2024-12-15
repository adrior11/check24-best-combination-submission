use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct SubsetDto {
    pub id: usize,
    pub elements: BTreeSet<usize>,
    pub cost: usize,
}
