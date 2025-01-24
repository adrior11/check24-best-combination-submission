use anyhow::Context;
use redis::{Client, Cmd, ErrorKind, RedisError};

/// Type alias for the Redis client.
pub type RedisClient = Client;

/// Initializes a connection to the Redis server.
///
/// This function establishes a Redis connection using the provided URL
/// and validates the connection by sending a `PING` command.
///
/// # Arguments
///
/// * `url` - A reference to a `String` containing the Redis server URL.
///
/// # Returns
///
/// Returns an `Ok(Client)` containing the initialized Redis client if the connection
/// is successfully established and validated. Returns an `Err(anyhow::Error)` otherwise.
///
/// # Errors
///
/// This function returns an `anyhow::Error` if:
/// - The Redis client cannot be created.
/// - The connection validation fails.
///
/// # Examples
///
/// ```no_run
/// use libs::caching::{init_redis, RedisClient};
///
/// # #[tokio::main]
/// # async fn main() -> anyhow::Result<()> {
/// let redis_url = "redis://localhost:6379".to_string();
/// let redis_client: RedisClient = init_redis(&redis_url).await?;
/// println!("Redis connection established successfully.");
/// # Ok(())
/// # }
/// ```
pub async fn init_redis(url: &String) -> anyhow::Result<Client> {
    let client = Client::open(url.to_owned()).context("Failed to connect to Redis")?;

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
async fn validate_redis_connection(client: &Client) -> redis::RedisResult<()> {
    let mut connection = client.get_multiplexed_tokio_connection().await?;

    let response: String = Cmd::new().arg("PING").query_async(&mut connection).await?;

    if response != "PONG" {
        return Err(RedisError::from((ErrorKind::IoError, "Ping failed")));
    }

    Ok(())
}
