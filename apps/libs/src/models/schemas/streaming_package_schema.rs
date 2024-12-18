use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::models::util::deserialize_optional_numeric_from_string;

#[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Debug, Validate, Ord, PartialOrd)]
pub struct StreamingPackageSchema {
    #[serde(rename = "_id")]
    pub id: ObjectId,

    #[validate(range(min = 1))]
    pub streaming_package_id: u32,

    #[validate(length(min = 1))]
    pub name: String,

    #[serde(deserialize_with = "deserialize_optional_numeric_from_string", default)]
    pub monthly_price_cents: Option<u16>,

    #[validate(range(min = 0))]
    pub monthly_price_yearly_subscription_in_cents: u16,
}

#[cfg(test)]
mod tests {
    use super::*;
    use mongodb::bson::oid;
    use validator::Validate;

    #[test]
    fn test_streaming_package_valid() {
        let package = StreamingPackageSchema {
            id: oid::ObjectId::new(),
            streaming_package_id: 1,
            name: "Premium Package".to_string(),
            monthly_price_cents: Some(999),
            monthly_price_yearly_subscription_in_cents: 11988,
        };

        assert!(package.validate().is_ok());
    }

    #[test]
    fn test_streaming_package_invalid_streaming_package_id() {
        let package = StreamingPackageSchema {
            id: oid::ObjectId::new(),
            streaming_package_id: 0,
            name: "Premium Package".to_string(),
            monthly_price_cents: Some(999),
            monthly_price_yearly_subscription_in_cents: 11988,
        };

        assert!(package.validate().is_err());
    }

    #[test]
    fn test_streaming_package_invalid_name_empty() {
        let package = StreamingPackageSchema {
            id: oid::ObjectId::new(),
            streaming_package_id: 1,
            name: "".to_string(),
            monthly_price_cents: Some(999),
            monthly_price_yearly_subscription_in_cents: 11988,
        };

        assert!(package.validate().is_err());
    }

    #[test]
    fn test_streaming_package_optional_monthly_price_cents() {
        let package = StreamingPackageSchema {
            id: oid::ObjectId::new(),
            streaming_package_id: 1,
            name: "Basic Package".to_string(),
            monthly_price_cents: None,
            monthly_price_yearly_subscription_in_cents: 11988,
        };

        assert!(package.validate().is_ok());
    }

    #[test]
    fn test_deserialize_valid_number_as_string() {
        let json_data = r#"
    {
        "_id": {"$oid": "60b8d295e1d4d8a6f4d1e1e1"},
        "streaming_package_id": 1,
        "name": "Package A",
        "monthly_price_cents": "999",
        "monthly_price_yearly_subscription_in_cents": 11988
    }
    "#;

        let package: Result<StreamingPackageSchema, _> = serde_json::from_str(json_data);
        assert!(package.is_ok());
        assert_eq!(package.unwrap().monthly_price_cents, Some(999));
    }

    #[test]
    fn test_deserialize_valid_number() {
        let json_data = r#"
    {
        "_id": {"$oid": "60b8d295e1d4d8a6f4d1e1e2"},
        "streaming_package_id": 2,
        "name": "Package B",
        "monthly_price_cents": 499,
        "monthly_price_yearly_subscription_in_cents": 5990
    }
    "#;

        let package: Result<StreamingPackageSchema, _> = serde_json::from_str(json_data);
        assert!(package.is_ok());
        assert_eq!(package.unwrap().monthly_price_cents, Some(499));
    }

    #[test]
    fn test_deserialize_empty_string() {
        let json_data = r#"
    {
        "_id": {"$oid": "60b8d295e1d4d8a6f4d1e1e3"},
        "streaming_package_id": 3,
        "name": "Package C",
        "monthly_price_cents": "",
        "monthly_price_yearly_subscription_in_cents": 0
    }
    "#;

        let package: Result<StreamingPackageSchema, _> = serde_json::from_str(json_data);
        assert!(package.is_ok());
        assert_eq!(package.unwrap().monthly_price_cents, None);
    }

    #[test]
    fn test_deserialize_null_value() {
        let json_data = r#"
    {
        "_id": {"$oid": "60b8d295e1d4d8a6f4d1e1e4"},
        "streaming_package_id": 4,
        "name": "Package D",
        "monthly_price_cents": null,
        "monthly_price_yearly_subscription_in_cents": 0
    }
    "#;

        let package: Result<StreamingPackageSchema, _> = serde_json::from_str(json_data);
        assert!(package.is_ok());
        assert_eq!(package.unwrap().monthly_price_cents, None);
    }

    #[test]
    fn test_deserialize_missing_field() {
        let json_data = r#"
    {
        "_id": {"$oid": "60b8d295e1d4d8a6f4d1e1e5"},
        "streaming_package_id": 5,
        "name": "Package E",
        "monthly_price_yearly_subscription_in_cents": 0
    }
    "#;

        let package: Result<StreamingPackageSchema, _> = serde_json::from_str(json_data);
        assert!(package.is_ok());
        assert_eq!(package.unwrap().monthly_price_cents, None);
    }

    #[test]
    fn test_deserialize_invalid_string() {
        let json_data = r#"
    {
        "_id": {"$oid": "60b8d295e1d4d8a6f4d1e1e6"},
        "streaming_package_id": 6,
        "name": "Package F",
        "monthly_price_cents": "abc",
        "monthly_price_yearly_subscription_in_cents": 0
    }
    "#;

        let package: Result<StreamingPackageSchema, _> = serde_json::from_str(json_data);
        assert!(package.is_err());
    }

    #[test]
    fn test_deserialize_invalid_type_boolean() {
        let json_data = r#"
    {
        "_id": {"$oid": "60b8d295e1d4d8a6f4d1e1e7"},
        "streaming_package_id": 7,
        "name": "Package G",
        "monthly_price_cents": true,
        "monthly_price_yearly_subscription_in_cents": 0
    }
    "#;

        let package: Result<StreamingPackageSchema, _> = serde_json::from_str(json_data);
        assert!(package.is_err());
    }
}
