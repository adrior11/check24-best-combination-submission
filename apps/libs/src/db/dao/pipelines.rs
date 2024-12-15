use mongodb::bson::{doc, Document};

/// Creates an aggregation pipeline to filter games by teams and project the game_id.
///
/// # Arguments
///
/// * `teams` - A slice of team names to filter `team_home` or `team_away`.
///
/// # Returns
///
/// A vector of `Document` representing the aggregation pipeline.
pub fn games_by_teams_pipeline(teams: &[String]) -> Vec<Document> {
    vec![
        doc! {
            "$match": {
                "$or": [
                    { "team_home": { "$in": teams } },
                    { "team_away": { "$in": teams } }
                ]
            }
        },
        doc! {
            "$project": {
                "_id": 0,
                "game_id": 1
            }
        },
    ]
}

/// Creates an aggregation pipeline for preprocessing subsets with filtered game_ids.
///
/// # Arguments
///
/// * `game_ids` - A slice of u32 representing the game_ids to filter.
///
/// # Returns
///
/// A vector of `Document` representing the aggregation pipeline.
pub fn preprocess_subsets_pipeline(game_ids: &[u32]) -> Vec<Document> {
    vec![
        doc! {
            "$lookup": doc! {
                "from": "bc_streaming_offer",
                "localField": "streaming_package_id",
                "foreignField": "streaming_package_id",
                "as": "offers",
                "pipeline": [
                    doc! {
                        "$match": doc! {
                            "game_id": doc! {
                                "$in":  game_ids
                            }
                        }
                    },
                    doc! {
                        "$project": doc! {
                            "_id": 0,
                            "game_id": 1
                        }
                    }
                ]
            }
        },
        doc! {
            "$match": doc! {
                "offers": doc! {
                    "$ne": []
                }
            }
        },
        doc! {
            "$project": doc! {
                "_id": 0,
                "id": "$streaming_package_id",
                "cost": doc! {
                    "$cond": doc! {
                        "if": doc! {
                            "$eq": [
                                "$monthly_price_cents",
                                ""
                            ]
                        },
                        "then": 100000,
                        "else": doc! {
                            "$toInt": "$monthly_price_cents"
                        }
                    }
                },
                "elements": doc! {
                    "$map": doc! {
                        "input": "$offers",
                        "as": "offer",
                        "in": "$$offer.game_id"
                    }
                }
            }
        },
    ]
}
