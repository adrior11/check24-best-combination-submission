use std::convert::TryFrom;

use serde::{de, Deserialize, Deserializer};
use serde_json::Value;

/// Deserializes an optional numeric value from a string or number representation.
///
/// This utility function is used with Serde to handle cases where a numeric value might
/// be represented as a string, a numeric literal, or `null`. The function supports converting
/// strings and numbers into the target numeric type, or returning `None` if the value is
/// empty, `null`, or invalid.
///
/// # Type Parameters
///
/// * `'de` - The lifetime of the deserializer.
/// * `D` - The deserializer type.
/// * `T` - The target numeric type, which must implement `TryFrom<u64>`, `FromStr`, and `Default`.
///
/// # Arguments
///
/// * `deserializer` - The deserializer provided by Serde.
///
/// # Returns
///
/// Returns a `Result<Option<T>, D::Error>` where:
/// - `Ok(Some(T))` if a valid numeric value is successfully deserialized.
/// - `Ok(None)` if the input is empty, `null`, or cannot be converted to the target type.
/// - `Err(D::Error)` if the input type is unsupported or cannot be deserialized.
///
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
