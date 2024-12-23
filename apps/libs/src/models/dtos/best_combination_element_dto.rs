use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub struct BestCombinationElementDto {
    pub game_id: usize,
    pub tournament_name: String,
    pub live: u8,
    pub highlights: u8,
}

impl BestCombinationElementDto {
    pub fn new(game_id: usize, tournament_name: &str, live: u8, highlights: u8) -> Self {
        BestCombinationElementDto {
            game_id,
            tournament_name: tournament_name.to_string(),
            live,
            highlights,
        }
    }
}
