use std::hash::{Hash, Hasher};

use anyhow::Context;
use redis::AsyncCommands;
use serde::{Deserialize, Serialize};

/// Time-To-Live (TTL) for cache entries in seconds.
/// Preset to 1 Week: 7 days * 24 hours * 60 minutes * 60 seconds
const CACHE_TTL: u64 = 7 * 24 * 60 * 60;

/// A generic cache entry structure for storing key-value pairs in Redis.
///
/// # Type Parameters
///
/// * `T` - The type of the value to be cached. This must implement `Serialize` and `Deserialize`.
#[derive(Serialize, Deserialize, Debug)]
pub struct CacheEntry<T> {
    pub key: Vec<usize>,
    pub value: T,
}

/// Caches a entry in Redis.
///
/// This function stores a key-value pair in Redis with an preset TTL of 1 week.
/// The key is generated from the provided identifiers, and the value is serialized into JSON format.
///
/// # Arguments
///
/// * `redis_client` - A reference to the Redis client used to connect to the Redis server.
/// * `key` - A vector of unique identifiers used to generate the cache key.
/// * `value` - The value to cache. This can be any type that implements `Serialize`.
///
/// # Errors
///
/// Returns an `anyhow::Error` if:
/// - A connection to Redis cannot be established.
/// - The value cannot be serialized into JSON.
/// - The Redis `SET` operation fails.
///
/// # Examples
///
/// ```no_run
/// # use anyhow;
/// use libs::caching::{cache_entry, CacheEntry, init_redis, RedisClient};
///
/// # #[tokio::main]
/// # async fn main() -> anyhow::Result<()> {
/// let redis_url = "redis://localhost:6379".to_string();
/// let redis_client: RedisClient = init_redis(&redis_url).await?;
/// let cache_key = vec![1, 2, 3];
/// let cache_value = "Hello World!";
///
/// cache_entry(&redis_client, cache_key, cache_value).await?;
/// # Ok(())
/// # }
pub async fn cache_entry<T>(
    redis_client: &redis::Client,
    key: Vec<usize>,
    value: T,
) -> anyhow::Result<()>
where
    T: Serialize,
{
    let mut connection = redis_client.get_multiplexed_tokio_connection().await?;

    let cache_key = format!("cache:{}", hash_input(&key));
    let cache_value = serde_json::to_string(&CacheEntry { key, value })
        .context("Failed to serialize cache value")?;

    let _: () = connection.set_ex(cache_key, cache_value, CACHE_TTL).await?;
    Ok(())
}

/// Retrieves a cached entry from Redis.
///
/// This function fetches a cached key-value pair from Redis, deserializes it,
/// and returns the result if found.
///
/// # Arguments
///
/// * `redis_client` - A reference to the Redis client used to connect to the Redis server.
/// * `key` - A slice of unique identifiers to generate the cache key.
///
/// # Returns
///
/// * `Ok(Some(CacheEntry<T>))` if a cached entry is found and successfully deserialized.
/// * `Ok(None)` if no cached entry exists for the provided key.
/// * `Err(anyhow::Error)` if there is an issue retrieving or deserializing the cached data.
///
/// # Errors
///
/// Returns an `anyhow::Error` if:
/// - A connection to Redis cannot be established.
/// - The value cannot be deserialized from JSON.
///
/// # Examples
///
/// ```no_run
/// # use anyhow;
/// use libs::caching::{get_cached_entry, CacheEntry, init_redis, RedisClient};
///
/// # #[tokio::main]
/// # async fn main() -> anyhow::Result<()> {
/// let redis_url = "redis://localhost:6379".to_string();
/// let redis_client: RedisClient = init_redis(&redis_url).await?;
/// let cache_key = vec![1, 2, 3];
///
/// if let Some(cached) = get_cached_entry::<String>(&redis_client, &cache_key).await? {
///     println!("Cached value: {:?}", cached.value);
/// } else {
///     println!("No cache entry found.");
/// }
/// # Ok(())
/// # }
pub async fn get_cached_entry<T>(
    redis_client: &redis::Client,
    key: &[usize],
) -> anyhow::Result<Option<CacheEntry<T>>>
where
    T: for<'de> Deserialize<'de>,
{
    let mut connection = redis_client.get_multiplexed_tokio_connection().await?;

    let cache_key = format!("cache:{}", hash_input(key));
    let cache_value: Option<String> = connection.get(cache_key).await.ok();

    if let Some(value) = cache_value {
        let entry: CacheEntry<T> =
            serde_json::from_str(&value).context("Failed to deserialize cache value")?;
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
fn hash_input(input_ids: &[usize]) -> String {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    let mut sorted_ids = input_ids.to_vec();
    sorted_ids.sort(); // Sort to ensure order independence
    sorted_ids.iter().for_each(|id| id.hash(&mut hasher));
    hasher.finish().to_string()
}
