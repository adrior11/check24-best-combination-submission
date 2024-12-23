use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use super::BestCombinationElementDto;
use crate::models::util::deserialize_optional_numeric_from_string;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct BestCombinationSubsetDto {
    pub streaming_package_id: usize,
    pub elements: BTreeSet<BestCombinationElementDto>,
    #[serde(deserialize_with = "deserialize_optional_numeric_from_string", default)]
    pub monthly_price_cents: Option<usize>,
    pub monthly_price_yearly_subscription_in_cents: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_valid_number_as_string() {
        let json_data = r#"
    {
        "streaming_package_id": 1,
            "elements": [
                {
                    "game_id": 1,
                    "tournament_name": "T1",
                    "live": 1,
                    "highlights": 0
                },
                {
                    "game_id": 2,
                    "tournament_name": "T2",
                    "live": 0,
                    "highlights": 1
                }
            ],
        "monthly_price_cents": "999",
        "monthly_price_yearly_subscription_in_cents": 11988
    }
    "#;

        let package: Result<BestCombinationSubsetDto, _> = serde_json::from_str(json_data);
        assert!(package.is_ok());
        assert_eq!(package.unwrap().monthly_price_cents, Some(999));
    }

    #[test]
    fn test_deserialize_valid_number() {
        let json_data = r#"
        {
            "streaming_package_id": 1,
            "elements": [
                {
                    "game_id": 1,
                    "tournament_name": "T1",
                    "live": 1,
                    "highlights": 0
                },
                {
                    "game_id": 2,
                    "tournament_name": "T2",
                    "live": 0,
                    "highlights": 1
                }
            ],
            "monthly_price_cents": 999,
            "monthly_price_yearly_subscription_in_cents": 11988
        }
        "#;

        let package: Result<BestCombinationSubsetDto, _> = serde_json::from_str(json_data);
        assert!(package.is_ok());
        assert_eq!(package.unwrap().monthly_price_cents, Some(999));
    }

    #[test]
    fn test_deserialize_empty_string() {
        let json_data = r#"
        {
            "streaming_package_id": 1,
            "elements": [
                {
                    "game_id": 1,
                    "tournament_name": "T1",
                    "live": 1,
                    "highlights": 0
                },
                {
                    "game_id": 2,
                    "tournament_name": "T2",
                    "live": 0,
                    "highlights": 1
                }
            ],
            "monthly_price_cents": "",
            "monthly_price_yearly_subscription_in_cents": 11988
        }
        "#;

        let package: Result<BestCombinationSubsetDto, _> = serde_json::from_str(json_data);
        assert!(package.is_ok());
        assert_eq!(package.unwrap().monthly_price_cents, None);
    }

    #[test]
    fn test_deserialize_null_value() {
        let json_data = r#"
        {
            "streaming_package_id": 1,
            "elements": [
                {
                    "game_id": 1,
                    "tournament_name": "T1",
                    "live": 1,
                    "highlights": 0
                },
                {
                    "game_id": 2,
                    "tournament_name": "T2",
                    "live": 0,
                    "highlights": 1
                }
            ],
            "monthly_price_cents": null,
            "monthly_price_yearly_subscription_in_cents": 11988
        }
        "#;

        let package: Result<BestCombinationSubsetDto, _> = serde_json::from_str(json_data);
        assert!(package.is_ok());
        assert_eq!(package.unwrap().monthly_price_cents, None);
    }

    #[test]
    fn test_deserialize_missing_field() {
        let json_data = r#"
        {
            "streaming_package_id": 1,
            "elements": [
                {
                    "game_id": 1,
                    "tournament_name": "T1",
                    "live": 1,
                    "highlights": 0
                },
                {
                    "game_id": 2,
                    "tournament_name": "T2",
                    "live": 0,
                    "highlights": 1
                }
            ],
            "monthly_price_yearly_subscription_in_cents": 11988
        }
        "#;

        let package: Result<BestCombinationSubsetDto, _> = serde_json::from_str(json_data);
        dbg!(&package);
        assert!(package.is_ok());
        assert_eq!(package.unwrap().monthly_price_cents, None);
    }

    #[test]
    fn test_deserialize_invalid_string() {
        let json_data = r#"
        {
            "streaming_package_id": 1,
            "elements": [
                {
                    "game_id": 1,
                    "tournament_name": "T1",
                    "live": 1,
                    "highlights": 0
                },
                {
                    "game_id": 2,
                    "tournament_name": "T2",
                    "live": 0,
                    "highlights": 1
                }
            ],
            "monthly_price_cents": "abc",
            "monthly_price_yearly_subscription_in_cents": 11988
        }
        "#;

        let package: Result<BestCombinationSubsetDto, _> = serde_json::from_str(json_data);
        assert!(package.is_err());
    }
}
