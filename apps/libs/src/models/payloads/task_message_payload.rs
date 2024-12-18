use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct TaskMessagePayload {
    pub game_ids: BTreeSet<usize>,
    pub limit: usize,
}
