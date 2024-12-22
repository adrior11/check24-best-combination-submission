use mongodb::bson::{doc, Document};

pub fn project_game_id() -> Document {
    doc! { "$project": { "game_id": 1, "_id": 0 } }
}

//pub fn best_combination_coverage(game_ids: &[u32], package_ids: &[u32]) -> Vec<Document> {
//    vec![
//        doc! {
//            "game_id": {
//                "$in": game_ids
//            },
//        },
//        doc! {
//            "$lookup": doc! {
//                "from": "bc_streaming_offer",
//                "localField": "game_id",
//                "foreignField": "game_id",
//                "as": "offers",
//                "pipeline": [
//                    doc! {
//                        "$match": doc! {
//                            "streaming_package_id": doc! {
//                                "$in":  package_ids
//                            }
//                        }
//                    }
//                ]
//            }
//        },
//        doc! {
//            "$unwind": doc! {
//                "path": "$offers"
//            }
//        },
//        doc! {
//            "$group": doc! {
//                "_id": doc! {
//                    "tournament_name": "$tournament_name",
//                    "streaming_package_id": "$offers.streaming_package_id"
//                },
//                "total_games": doc! {
//                    "$sum": 1
//                },
//                "live_coverage": doc! {
//                    "$sum": doc! {
//                        "$cond": [
//                            "$offers.live",
//                            1,
//                            0
//                        ]
//                    }
//                },
//                "highlights_coverage": doc! {
//                    "$sum": doc! {
//                        "$cond": [
//                            "$offers.highlights",
//                            1,
//                            0
//                        ]
//                    }
//                }
//            }
//        },
//        doc! {
//            "$project": doc! {
//                "_id": 0,
//                "tournament_name": "$_id.tournament_name",
//                "streaming_package_id": "$_id.streaming_package_id",
//                "total_games": 1,
//                "live_coverage": 1,
//                "highlights_coverage": 1
//            }
//        },
//    ]
//}
//
//pub fn games_by_teams_pipeline(teams: &[String]) -> Vec<Document> {
//    vec![
//        doc! {
//            "$match": {
//                "$or": [
//                    { "team_home": { "$in": teams } },
//                    { "team_away": { "$in": teams } }
//                ]
//            }
//        },
//        doc! {
//            "$project": {
//                "_id": 0,
//                "game_id": 1
//            }
//        },
//    ]
//}

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
                            "tournament_name": "$game.tournament_name"
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
                            "$$offer.tournament_name"
                        ]
                    }
                }
            }
        },
    ]
}
