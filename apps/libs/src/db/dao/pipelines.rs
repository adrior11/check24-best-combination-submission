use mongodb::bson::{doc, Document};

pub fn project_game_id() -> Document {
    doc! { "$project": { "game_id": 1, "_id": 0 } }
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
