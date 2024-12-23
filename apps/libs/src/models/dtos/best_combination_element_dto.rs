use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub struct BestCombinationElementDto {
    pub game_id: usize,
    pub tournament_name: String,
    pub live: u8,
    pub highlights: u8,
}
