use serde::{Deserialize, Serialize};

use crate::caching::CompositeKey;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct TaskMessagePayload {
    pub game_ids: Vec<usize>,
    pub limit: usize,
}

impl From<CompositeKey> for TaskMessagePayload {
    fn from(o: CompositeKey) -> Self {
        TaskMessagePayload {
            game_ids: o.ids,
            limit: o.opts.limit,
        }
    }
}
