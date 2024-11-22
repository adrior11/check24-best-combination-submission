use mongodb::bson::oid;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Validate)]
pub struct StreamingPackage {
    #[serde(rename = "_id")]
    pub id: oid::ObjectId,

    #[validate(range(min = 1))]
    pub streaming_package_id: u8,

    #[validate(length(min = 1))]
    pub name: String,

    pub monthly_price_cents: Option<u16>,

    #[validate(range(min = 0))]
    pub monthly_price_yearly_subscriptions_in_cents: u16,
}

#[cfg(test)]
mod tests {
    use super::*;
    use mongodb::bson::oid;
    use validator::Validate;

    #[test]
    fn test_streaming_package_valid() {
        let package = StreamingPackage {
            id: oid::ObjectId::new(),
            streaming_package_id: 1,
            name: "Premium Package".to_string(),
            monthly_price_cents: Some(999),
            monthly_price_yearly_subscriptions_in_cents: 11988,
        };

        assert!(package.validate().is_ok());
    }

    #[test]
    fn test_streaming_package_invalid_streaming_package_id() {
        let package = StreamingPackage {
            id: oid::ObjectId::new(),
            streaming_package_id: 0,
            name: "Premium Package".to_string(),
            monthly_price_cents: Some(999),
            monthly_price_yearly_subscriptions_in_cents: 11988,
        };

        assert!(package.validate().is_err());
    }

    #[test]
    fn test_streaming_package_invalid_name_empty() {
        let package = StreamingPackage {
            id: oid::ObjectId::new(),
            streaming_package_id: 1,
            name: "".to_string(),
            monthly_price_cents: Some(999),
            monthly_price_yearly_subscriptions_in_cents: 11988,
        };

        assert!(package.validate().is_err());
    }

    #[test]
    fn test_streaming_package_optional_monthly_price_cents() {
        let package = StreamingPackage {
            id: oid::ObjectId::new(),
            streaming_package_id: 1,
            name: "Basic Package".to_string(),
            monthly_price_cents: None,
            monthly_price_yearly_subscriptions_in_cents: 11988,
        };

        assert!(package.validate().is_ok());
    }
}
