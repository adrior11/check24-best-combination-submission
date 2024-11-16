#[cfg(test)]
mod tests {
    use crate::core::models::streaming_package_schema;
    use mongodb::bson::oid;
    use validator::Validate;

    #[test]
    fn test_streaming_package_valid() {
        let package = streaming_package_schema::StreamingPackage {
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
        let package = streaming_package_schema::StreamingPackage {
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
        let package = streaming_package_schema::StreamingPackage {
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
        let package = streaming_package_schema::StreamingPackage {
            id: oid::ObjectId::new(),
            streaming_package_id: 1,
            name: "Basic Package".to_string(),
            monthly_price_cents: None,
            monthly_price_yearly_subscriptions_in_cents: 11988,
        };

        assert!(package.validate().is_ok());
    }
}
