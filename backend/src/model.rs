use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use serde::de::{self, Deserializer};
use chrono::{DateTime, Utc};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Game {
    #[serde(rename = "_id")]
    pub mongo_id: Option<ObjectId>,
    pub id: usize,
    pub team_home: String,
    pub team_away: String,
    #[serde(deserialize_with = "from_datetime_str")]
    pub starts_at: DateTime<Utc>,
    pub tournament_name: String
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct StreamingPackage {
    #[serde(rename = "_id")]
    pub mongo_id: Option<ObjectId>,
    pub id: usize,
    pub name: String,
    pub monthly_price_cents: Option<usize>,
    pub monthly_price_yearly_subscription_in_cents: usize,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct StreamingOffer {
    #[serde(rename = "_id")]
    pub mongo_id: Option<ObjectId>,
    pub game_id: usize,
    pub streaming_package_id: usize,
    #[serde(deserialize_with = "bool_from_int")]
    pub live: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub highlights: bool
}

fn from_datetime_str<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where 
    D: Deserializer<'de>
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    DateTime::parse_from_rfc3339(s)
        .map(|dt| dt.with_timezone(&Utc))
        .map_err(de::Error::custom)
}

fn bool_from_int<'de, D>(deserializer: D) -> Result<bool, D::Error>
where 
    D: Deserializer<'de>,
{
    let value: u8 = Deserialize::deserialize(deserializer)?;
    Ok(value != 0)
}
