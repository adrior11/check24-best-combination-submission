use std::convert::TryFrom;

use serde::{de, Deserialize, Deserializer};
use serde_json::Value;

pub fn deserialize_optional_numeric_from_string<'de, D, T>(
    deserializer: D,
) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: TryFrom<u64> + std::str::FromStr + Default,
    <T as std::str::FromStr>::Err: std::fmt::Display,
{
    let value = Value::deserialize(deserializer)?;

    match value {
        Value::String(s) if !s.trim().is_empty() => {
            s.parse::<T>().map(Some).map_err(de::Error::custom)
        }
        Value::String(_) => Ok(None),
        Value::Number(num) => num
            .as_u64()
            .and_then(|n| T::try_from(n).ok())
            .map(Some)
            .ok_or_else(|| de::Error::custom("Invalid number for target type")),
        Value::Null => Ok(None),
        _ => Err(de::Error::custom("Invalid type for target type")),
    }
}
