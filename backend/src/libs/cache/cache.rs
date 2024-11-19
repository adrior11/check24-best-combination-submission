use std::env;

pub async fn init_redis() -> redis::RedisResult<redis::Client> {
    let redis_url = env::var("REDIS_URL").expect("REDIS_URL must be set in .env");

    let client = redis::Client::open(redis_url).expect("Failed to create Redis client");

    validate_redis_connection(&client).await?;

    Ok(client)
}

async fn validate_redis_connection(client: &redis::Client) -> redis::RedisResult<()> {
    let mut connection = client.get_multiplexed_tokio_connection().await?;

    let response: String = redis::Cmd::new()
        .arg("PING")
        .query_async(&mut connection)
        .await?;

    if response != "PONG" {
        return Err(redis::RedisError::from((
            redis::ErrorKind::IoError,
            "Ping failed",
        )));
    }

    Ok(())
}
