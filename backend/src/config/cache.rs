use std::env;

pub async fn init_redis() -> redis::RedisResult<redis::Client> {
    dotenv::dotenv().ok();

    let redis_url = env::var("REDIS_URL").expect("REDIS_URL must be set in .env");

    let client = redis::Client::open(redis_url).expect("Failed to create Redis client");

    Ok(client)
}
