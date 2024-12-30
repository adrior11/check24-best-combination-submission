// TODO: Add chrono for date range filters
use mongodb::bson::oid::ObjectId;
use once_cell::sync;
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Regex pattern to validate the `starts_at` field, ensuring it follows the format `YYYY-MM-DD HH:MM:SS`.
/// - The regex enforces valid ranges for each component but does not account for leap years or month-specific day limits.
const STARTS_AT_REGEX: &str = r"^(20\d{2})-(0[1-9]|1[0-2])-(0[1-9]|[12][0-9]|3[01])\s(0[0-9]|1[0-9]|2[0-3]):([0-5][0-9]):([0-5][0-9])$";

static STARTS_AT: sync::Lazy<regex::Regex> =
    sync::Lazy::new(|| regex::Regex::new(STARTS_AT_REGEX).unwrap());

#[derive(Clone, Serialize, Deserialize, Debug, Validate)]
pub struct GameSchema {
    #[serde(rename = "_id")]
    pub id: ObjectId,

    #[validate(range(min = 1))]
    pub game_id: u32,

    #[validate(length(min = 1))]
    pub team_away: String,

    #[validate(length(min = 1))]
    pub team_home: String,

    #[validate(regex(path = "*STARTS_AT"))]
    pub starts_at: String,

    #[validate(length(min = 1))]
    pub tournament_name: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use mongodb::bson::oid;
    use validator::Validate;

    #[test]
    fn test_game_validation_valid() {
        let game = GameSchema {
            id: oid::ObjectId::new(),
            game_id: 1,
            team_away: "TEAM A".to_string(),
            team_home: "TEAM B".to_string(),
            starts_at: "2024-06-14 19:00:00".to_string(),
            tournament_name: "TOURNAMENT X".to_string(),
        };

        assert!(game.validate().is_ok());
    }

    #[test]
    fn test_game_validation_invalid_starts_at() {
        let game = GameSchema {
            id: oid::ObjectId::new(),
            game_id: 1,
            team_away: "Team A".to_string(),
            team_home: "Team B".to_string(),
            starts_at: "14-06-2024 19:00:00".to_string(),
            tournament_name: "Tournament X".to_string(),
        };

        assert!(game.validate().is_err());
    }

    #[test]
    fn test_game_validation_invalid_starts_at_time() {
        let game = GameSchema {
            id: oid::ObjectId::new(),
            game_id: 1,
            team_away: "Team A".to_string(),
            team_home: "Team B".to_string(),
            starts_at: "2024-06-14 25:61:60".to_string(),
            tournament_name: "Tournament X".to_string(),
        };

        assert!(game.validate().is_err());
    }

    #[test]
    fn test_game_validation_invalid_starts_at_month() {
        let game = GameSchema {
            id: oid::ObjectId::new(),
            game_id: 1,
            team_away: "Team A".to_string(),
            team_home: "Team B".to_string(),
            starts_at: "2024-13-14 19:00:00".to_string(),
            tournament_name: "Tournament X".to_string(),
        };

        assert!(game.validate().is_err());
    }
}
