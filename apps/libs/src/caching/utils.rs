use anyhow::Context;
use redis::AsyncCommands;
use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};

/// Represents a cached entry containing input and corresponding output identifiers.
///
/// This struct is serialized and stored in Redis to cache the results of computations for API calls.
/// It includes:
/// - `input_ids`: A vector of input identifiers.
/// - `output_ids`: A vector of vectors containing output identifiers corresponding to each input.
#[derive(Serialize, Deserialize, Debug)]
pub struct CacheEntry {
    pub input_ids: Vec<u32>,
    pub output_ids: Vec<Vec<u32>>,
}

/// Caches the provided input and output identifiers in Redis.
///
/// This asynchronous function serializes the `CacheEntry` and stores it in Redis using a key derived from the input identifiers.
///
/// # Arguments
///
/// * `redis_client` - A reference to the Redis client used to connect to the Redis server.
/// * `input_ids` - A vector of input identifiers to be cached.
/// * `output_ids` - A vector of vectors containing output identifiers corresponding to each input.
///
/// # Errors
///
/// Returns an `anyhow::Error` if:
/// - There is an issue obtaining a connection to Redis.
/// - Serialization of the `CacheEntry` fails.
/// - Storing the data in Redis fails.
pub async fn cache_result(
    redis_client: &redis::Client,
    input_ids: Vec<u32>,
    output_ids: Vec<Vec<u32>>,
) -> anyhow::Result<()> {
    let mut connection = redis_client.get_multiplexed_tokio_connection().await?;

    let cache_key = format!("cache:{}", hash_input(&input_ids));

    let cache_value = serde_json::to_string(&CacheEntry {
        input_ids,
        output_ids,
    })?;

    let _: () = connection.set(cache_key, cache_value).await?;

    Ok(())
}

/// Retrieves a cached `CacheEntry` from Redis based on the provided input identifiers.
///
/// This asynchronous function fetches the cached data from Redis, deserializes it, and returns the `CacheEntry` if found.
///
/// # Arguments
///
/// * `redis_client` - A reference to the Redis client used to connect to the Redis server.
/// * `input_ids` - A slice of input identifiers to retrieve the corresponding cached entry.
///
/// # Returns
///
/// * `Ok(Some(CacheEntry))` if a cached entry is found and successfully deserialized.
/// * `Ok(None)` if no cached entry exists for the provided input identifiers.
/// * `Err(anyhow::Error)` if there is an issue retrieving or deserializing the cached data.
///
/// # Errors
///
/// Returns an `anyhow::Error` if:
/// - There is an issue obtaining a connection to Redis.
/// - Deserialization of the cached data fails.
pub async fn get_cached_result(
    redis_client: &redis::Client,
    input_ids: &[u32],
) -> anyhow::Result<Option<CacheEntry>> {
    let mut connection = redis_client.get_multiplexed_tokio_connection().await?;

    let cache_key = format!("cache:{}", hash_input(input_ids));

    let cache_value: Option<String> = connection.get(cache_key).await.ok();

    if let Some(value) = cache_value {
        let entry: CacheEntry =
            serde_json::from_str(&value).context("Failed to parse cache entry")?;
        Ok(Some(entry))
    } else {
        Ok(None)
    }
}

/// Generates a hash string from the provided input identifiers.
///
/// This function creates a hash based on the input IDs, which is used as part of the Redis cache key.
///
/// # Arguments
///
/// * `input_ids` - A slice of input identifiers to hash.
///
/// # Returns
///
/// A `String` representing the hash of the input identifiers.
///
fn hash_input(input_ids: &[u32]) -> String {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    input_ids.iter().for_each(|id| id.hash(&mut hasher));
    hasher.finish().to_string()
}
