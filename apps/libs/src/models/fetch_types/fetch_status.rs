use async_graphql::Enum;
use serde::{Deserialize, Serialize};

#[derive(Enum, Copy, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum FetchStatus {
    Ready,
    Processing,
    Error,
}
