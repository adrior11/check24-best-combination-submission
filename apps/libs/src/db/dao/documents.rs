use mongodb::bson::{doc, Bson::Null, Document};

pub fn filter_teams(teams: &[String]) -> Document {
    doc! {
        "$or": [
            { "team_home": { "$in": teams } },
            { "team_away": { "$in": teams } },
        ]
    }
}

pub fn filter_tournaments(tournaments: &[String]) -> Document {
    doc! {
        "tournament_name": { "$in": tournaments }
    }
}

pub fn project_game_id() -> Document {
    doc! { "$project": { "game_id": 1, "_id": 0 } }
}

pub fn aggregate_teams_pipeline() -> Vec<Document> {
    vec![
        doc! {
            "$project": doc! {
                "combined_teams": doc! {
                    "$setUnion": [
                        [
                            "$team_away"
                        ],
                        [
                            "$team_home"
                        ]
                    ]
                }
            }
        },
        doc! {
            "$unwind": doc! {
                "path": "$combined_teams"
            }
        },
        doc! {
            "$group": doc! {
                "_id": Null,
                "teams": doc! {
                    "$addToSet": "$combined_teams"
                }
            }
        },
        doc! {
            "$project": doc! {
                "_id": 0,
                "teams": 1
            }
        },
    ]
}

pub fn aggregate_tournaments_pipeline() -> Vec<Document> {
    vec![
        doc! {
            "$group": doc! {
                "_id": Null,
                "tournaments": doc! {
                    "$addToSet": "$tournament_name"
                }
            }
        },
        doc! {
            "$project": doc! {
                "_id": 0,
                "tournaments": 1
            }
        },
    ]
}

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
                        "$lookup": doc! {
                            "from": "bc_game",
                            "localField": "game_id",
                            "foreignField": "game_id",
                            "as": "game",
                            "pipeline": [
                                doc! {
                                    "$project": doc! {
                                        "_id": 0,
                                        "tournament_name": 1
                                    }
                                }
                            ]
                        }
                    },
                    doc! {
                        "$unwind": "$game"
                    },
                    doc! {
                        "$project": doc! {
                            "_id": 0,
                            "game_id": 1,
                            "tournament_name": "$game.tournament_name",
                            "live": 1,
                            "highlights": 1,
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
                "streaming_package_id": 1,
                "monthly_price_cents": 1,
                "monthly_price_yearly_subscription_in_cents": 1,
                "elements": doc! {
                    "$map": doc! {
                        "input": "$offers",
                        "as": "offer",
                        "in": [
                            "$$offer.game_id",
                            "$$offer.tournament_name",
                            "$$offer.live",
                            "$$offer.highlights"
                        ]
                    }
                }
            }
        },
    ]
}
