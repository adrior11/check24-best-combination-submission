mod client;
mod utils;

pub use client::{init_redis, RedisClient};
pub use utils::{cache_result, get_cached_result};
