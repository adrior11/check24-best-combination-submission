use anyhow::Context;

pub async fn init_redis(url: &String) -> anyhow::Result<redis::Client> {
    let client = redis::Client::open(url.to_owned()).context("Failed to connect to Redis")?;

    validate_redis_connection(&client)
        .await
        .context("Failed to validate Redis connection")?;

    Ok(client)
}

/// Validates the Redis connection by sending a `PING` command.
///
/// This asynchronous function ensures that the Redis server is responsive and functioning correctly.
///
/// # Arguments
///
/// * `client` - A reference to the Redis client used to establish the connection.
///
/// # Errors
///
/// Returns a `redis::RedisError` if:
/// - The `PING` command does not return `"PONG"`.
/// - There is an issue sending the `PING` command or receiving the response.
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
