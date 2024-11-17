#[cfg(test)]
mod tests {
    use crate::common::models::game_schema;
    use mongodb::bson::oid;
    use validator::Validate;

    #[test]
    fn test_game_validation_valid() {
        let game = game_schema::Game {
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
        let game = game_schema::Game {
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
        let game = game_schema::Game {
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
        let game = game_schema::Game {
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
