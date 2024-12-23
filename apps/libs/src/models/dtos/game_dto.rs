use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};

use crate::models::schemas::GameSchema;

#[derive(SimpleObject, Clone, Serialize, Deserialize, Debug)]
pub struct GameDto {
    pub game_id: u32,
    pub team_away: String,
    pub team_home: String,
    pub starts_at: String,
    pub tournament_name: String,
}

impl From<GameSchema> for GameDto {
    fn from(o: GameSchema) -> Self {
        GameDto {
            game_id: o.game_id,
            team_away: o.team_away,
            team_home: o.team_home,
            starts_at: o.starts_at,
            tournament_name: o.tournament_name,
        }
    }
}
