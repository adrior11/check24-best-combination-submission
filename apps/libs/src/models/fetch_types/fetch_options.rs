use std::{
    fmt::{Display, Formatter, Result},
    hash::{Hash, Hasher},
};

use async_graphql::InputObject;
use serde::{Deserialize, Serialize};

#[derive(InputObject, Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct FetchOptions {
    #[graphql(default = 1)]
    pub limit: usize,
}

impl FetchOptions {
    pub fn new(limit: usize) -> FetchOptions {
        FetchOptions { limit }
    }
}

impl Display for FetchOptions {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.limit)
    }
}

impl Hash for FetchOptions {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.limit.hash(state);
    }
}
