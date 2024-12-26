use std::hash::{DefaultHasher, Hash, Hasher};

use serde::{Deserialize, Serialize};

use super::StableHash;
use crate::models::{fetch_types::FetchOptions, payloads::TaskMessagePayload};

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct CompositeKey {
    pub ids: Vec<usize>,
    pub opts: FetchOptions,
}

impl CompositeKey {
    pub fn new(ids: Vec<usize>, opts: FetchOptions) -> CompositeKey {
        CompositeKey { ids, opts }
    }
}

impl From<TaskMessagePayload> for CompositeKey {
    fn from(o: TaskMessagePayload) -> Self {
        CompositeKey {
            ids: o.game_ids,
            opts: FetchOptions { limit: o.limit },
        }
    }
}

impl StableHash for CompositeKey {
    fn stable_hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();

        let mut sorted_ids = self.ids.clone();
        sorted_ids.sort();
        sorted_ids.hash(&mut hasher);

        self.opts.hash(&mut hasher);

        hasher.finish()
    }
}
