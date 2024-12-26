/// A trait for generating a stable (deterministic) hash of a key.
///
/// # Overview
///
/// This trait is intended to be implemented by any type that needs to serve
/// as a cache key. The core requirement is that `stable_hash()` must always
/// produce the same value for logically equivalent inputs. For instance,
/// if your key contains a collection (like a `Vec<usize>`), you should ensure
/// the collection is sorted or otherwise normalized within the `stable_hash()`
/// method so that it’s order-independent.
pub trait StableHash {
    /// Produces a consistent hash for this key, ensuring
    /// that any order-independent elements (e.g. Vec of IDs)
    /// are sorted or otherwise canonicalized before hashing.
    fn stable_hash(&self) -> u64;
}

/// Creates a Redis key string by prefixing `"cache:"` to the key’s stable hash.
///
/// # Overview
///
/// This is a simple helper that:
/// 1. Calls `stable_hash()` on the input,
/// 2. Converts the resulting `u64` into a string,
/// 3. Prefixes it with `"cache:"`.
pub fn hash_key(key: &impl StableHash) -> String {
    format!("cache:{}", key.stable_hash())
}
