use mongodb::bson::{self, Document};

pub fn build_match_query(
    teams: Option<Vec<String>>,
    tournaments: Option<Vec<String>>,
) -> Option<Document> {
    let mut match_conditions = vec![];

    if let Some(teams) = teams {
        match_conditions.push(bson::doc! {
            "$or": [
                { "team_home": { "$in": &teams } },
                { "team_away": { "$in": &teams } }
            ]
        });
    }

    if let Some(tournaments) = tournaments {
        match_conditions.push(bson::doc! {
            "tournament_name": { "$in": &tournaments }
        });
    }

    if match_conditions.is_empty() {
        None
    } else if match_conditions.len() == 1 {
        Some(match_conditions.remove(0))
    } else {
        Some(bson::doc! { "$or": match_conditions })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_match_query() {
        let teams = Some(vec!["Team A".to_string(), "Team B".to_string()]);
        let tournaments = Some(vec!["Tournament 1".to_string()]);

        let query = build_match_query(teams, tournaments).unwrap();
        let expected_query = bson::doc! {
            "$or": [
                { "$or": [
                    { "team_home": { "$in": ["Team A", "Team B"] } },
                    { "team_away": { "$in": ["Team A", "Team B"] } }
                ]},
                { "tournament_name": { "$in": ["Tournament 1"] } }
            ]
        };

        assert_eq!(query, expected_query);
    }
}
