use mongodb::bson::oid;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Validate)]
pub struct StreamingOffer {
    #[serde(rename = "_id")]
    pub id: oid::ObjectId,

    #[validate(range(min = 1))]
    pub game_id: u8,

    #[validate(range(min = 1))]
    pub streaming_package_id: u8,

    #[validate(range(min = 0, max = 1))]
    pub live: u8,

    #[validate(range(min = 0, max = 1))]
    pub highlights: u8,
}

#[cfg(test)]
mod tests {
    use super::*;
    use mongodb::bson::oid;
    use validator::Validate;

    #[test]
    fn test_streaming_offer_valid() {
        let offer = StreamingOffer {
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
        let offer = StreamingOffer {
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
        let offer = StreamingOffer {
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
        let offer = StreamingOffer {
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
        let offer = StreamingOffer {
            id: oid::ObjectId::new(),
            game_id: 1,
            streaming_package_id: 1,
            live: 1,
            highlights: 2,
        };

        assert!(offer.validate().is_err());
    }
}
