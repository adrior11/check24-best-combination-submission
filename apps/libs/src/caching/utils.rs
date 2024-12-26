use anyhow::Context;
use redis::AsyncCommands;
use serde::{Deserialize, Serialize};

use super::{hash_key, StableHash};

/// Time-To-Live (TTL) for cache entries in seconds.
/// Preset to 1 Week: 7 days * 24 hours * 60 minutes * 60 seconds
const CACHE_TTL: u64 = 7 * 24 * 60 * 60;

/// Represents the value stored in the cache.
///
/// This enum enables distinguishing between values that are still being
/// computed and values that are fully computed and ready to be served.
///
/// # Variants
///
/// * `Processing` - Indicates that the requested data is not yet available
///    and is currently being computed or fetched in the background.
///    Clients may need to try again later.
///
/// * `Data(T)` - Holds the actual cached value of type `T`.
///
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum CacheValue<T> {
    Processing,
    Data(T),
}

/// A generic cache entry structure for storing key-value pairs in Redis.
///
/// # Overview
///
/// The `CacheEntry` pairs a unique key (of type `K`) with a `CacheValue` (of type `T`), which
/// can either be:
/// - `Processing` if the data is not yet computed or fetched, or
/// - `Data(T)` if the requested data is available.
///
/// This allows the cache to represent both pending and ready states in a single entry.
///
/// # Type Parameters
///
/// * `K` - The key type (e.g., a struct or other composite type) used to uniquely
///   identify cached data. Must be `Serialize`/`Deserialize`.
/// * `T` - The type of the cached value. Must also implement `Serialize`/`Deserialize`.
///
/// # Example
///
/// ```
/// use libs::caching::{CacheEntry, CacheValue};
///
/// // An example cache entry storing a string result
/// let entry_in_progress: CacheEntry<Vec<usize>, String> = CacheEntry {
///     key: vec![1, 2, 3],
///     value: CacheValue::Processing,
/// };
///
/// let entry_ready: CacheEntry<Vec<usize>, String> = CacheEntry {
///     key: vec![1, 2, 3],
///     value: CacheValue::Data("Cached result".to_string()),
/// };
/// ```
#[derive(Serialize, Deserialize, Debug)]
pub struct CacheEntry<K, T> {
    pub key: K,
    pub value: CacheValue<T>,
}

/// Caches a entry in Redis.
///
/// This function takes:
/// - A reference to a Redis client (`redis_client`),
/// - A key (`&K`, where `K` implements `StableHash`) used to generate the Redis key,
/// - A `CacheValue<T>` holding either in-progress or finalized data.
///
/// Internally, `cache_entry` calls [`hash_key`](fn.hash_key.html) to turn the key into a
/// unique string via the `StableHash` trait. It then serializes the entire
/// [`CacheEntry`](struct.CacheEntry.html) and stores it under that key with a preset TTL of 1 week.
///
/// # Arguments
///
/// * `redis_client` - A reference to the Redis client used to connect to the Redis server.
/// * `key` - A reference to a type that implements `StableHash` and `Serialize`.
/// * `value` - The [`CacheValue`](enum.CacheValue.html) to store (either `Processing` or `Data(T)`).
///
/// # Errors
///
/// This function returns an error if:
/// - It fails to establish a connection with Redis.
/// - The value cannot be serialized into JSON.
/// - The `SET` operation on Redis fails.
pub async fn cache_entry<K, T>(
    redis_client: &redis::Client,
    key: &K,
    value: CacheValue<T>,
) -> anyhow::Result<()>
where
    K: StableHash + Serialize,
    T: Serialize,
{
    let mut connection = redis_client.get_multiplexed_tokio_connection().await?;

    let cache_key = hash_key(key);
    let cache_value = serde_json::to_string(&CacheEntry { key, value })
        .context("Failed to serialize cache value")?;

    let _: () = connection.set_ex(cache_key, cache_value, CACHE_TTL).await?;
    Ok(())
}

/// Retrieves a cached entry from Redis by key.
///
/// Given a Redis client and a reference to a key, this function:
/// 1. Uses [`hash_key`](fn.hash_key.html) to compute a Redis key string.
/// 2. Attempts to fetch the corresponding JSON string from Redis.
/// 3. If found, deserializes it into a [`CacheEntry`](struct.CacheEntry.html).
/// 4. Returns `Some(entry)` on success or `None` if the key was not present in Redis.
///
/// # Arguments
///
/// * `redis_client` - A reference to the Redis client used to connect to the Redis server.
/// * `key` - A reference to a type that implements `StableHash` (and `Clone`/`Deserialize`) to look up.
///
/// # Returns
///
/// - `Ok(Some(CacheEntry<K, T>))` if a cached entry is found and successfully deserialized.
/// - `Ok(None)` if no entry is found for the given key.
/// - `Err(anyhow::Error)` if there is a problem retrieving or deserializing the cache data.
///
/// # Errors
///
/// An error may occur if:
/// - A connection to Redis cannot be established.
/// - The JSON deserialization fails (e.g., corrupted or incompatible data).
pub async fn get_cached_entry<K, T>(
    redis_client: &redis::Client,
    key: &K,
) -> anyhow::Result<Option<CacheEntry<K, T>>>
where
    K: StableHash + Clone + for<'de> Deserialize<'de>,
    T: for<'de> Deserialize<'de>,
{
    let mut connection = redis_client.get_multiplexed_tokio_connection().await?;

    let cache_key = hash_key(key);
    let cache_value: Option<String> = connection.get(cache_key).await.ok();

    if let Some(value) = cache_value {
        let entry: CacheEntry<K, T> =
            serde_json::from_str(&value).context("Failed to deserialize cache value")?;
        Ok(Some(entry))
    } else {
        Ok(None)
    }
}
