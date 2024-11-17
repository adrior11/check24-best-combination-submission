use mongodb::bson::oid;
use once_cell::sync;
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Regex pattern to validate the `starts_at` field, ensuring it follows the format `YYYY-MM-DD HH:MM:SS`.
/// - The regex enforces valid ranges for each component but does not account for leap years or month-specific day limits.
const STARTS_AT_REGEX: &str = r"^(20\d{2})-(0[1-9]|1[0-2])-(0[1-9]|[12][0-9]|3[01])\s(0[0-9]|1[0-9]|2[0-3]):([0-5][0-9]):([0-5][0-9])$";

static STARTS_AT: sync::Lazy<regex::Regex> =
    sync::Lazy::new(|| regex::Regex::new(STARTS_AT_REGEX).unwrap());

#[derive(Serialize, Deserialize, Debug, Validate)]
pub struct Game {
    #[serde(rename = "_id")]
    pub id: oid::ObjectId,

    #[validate(range(min = 1))]
    pub game_id: i32,

    #[validate(length(min = 1))]
    pub team_away: String,

    #[validate(length(min = 1))]
    pub team_home: String,

    #[validate(regex(path = "*STARTS_AT"))]
    pub starts_at: String,

    #[validate(length(min = 1))]
    pub tournament_name: String,
}
