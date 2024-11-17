#[cfg(test)]
mod tests {
    use crate::common::models::streaming_offer_schema;
    use mongodb::bson::oid;
    use validator::Validate;

    #[test]
    fn test_streaming_offer_valid() {
        let offer = streaming_offer_schema::StreamingOffer {
            id: oid::ObjectId::new(),
            game_id: 1,
            streaming_package_id: 1,
            live: 1,
            highlights: 0,
        };

        assert!(offer.validate().is_ok());
    }

    #[test]
    fn test_streaming_offer_invalid_game_id() {
        let offer = streaming_offer_schema::StreamingOffer {
            id: oid::ObjectId::new(),
            game_id: 0,
            streaming_package_id: 1,
            live: 1,
            highlights: 0,
        };

        assert!(offer.validate().is_err());
    }

    #[test]
    fn test_streaming_offer_invalid_streaming_package_id() {
        let offer = streaming_offer_schema::StreamingOffer {
            id: oid::ObjectId::new(),
            game_id: 1,
            streaming_package_id: 0,
            live: 1,
            highlights: 0,
        };

        assert!(offer.validate().is_err());
    }

    #[test]
    fn test_streaming_offer_invalid_live_value() {
        let offer = streaming_offer_schema::StreamingOffer {
            id: oid::ObjectId::new(),
            game_id: 1,
            streaming_package_id: 1,
            live: 2,
            highlights: 0,
        };

        assert!(offer.validate().is_err());
    }

    #[test]
    fn test_streaming_offer_invalid_highlights_value() {
        let offer = streaming_offer_schema::StreamingOffer {
            id: oid::ObjectId::new(),
            game_id: 1,
            streaming_package_id: 1,
            live: 1,
            highlights: 2,
        };

        assert!(offer.validate().is_err());
    }
}
