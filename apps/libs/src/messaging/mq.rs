use lapin::{
    options::{BasicConsumeOptions, BasicPublishOptions, QueueDeclareOptions},
    types::FieldTable,
    BasicProperties, Channel, Connection, ConnectionProperties, Consumer,
};
use serde::Serialize;

/// A type alias for RabbitMQ's [`Channel`](https://docs.rs/lapin/latest/lapin/struct.Channel.html).
///
/// `MqChannel` represents a dedicated channel for messaging operations within the application.
/// It is used to interact with RabbitMQ for publishing and consuming messages.
///
/// # Example
///
/// ```rust
/// use libs::messaging::MqChannel;
///
/// async fn example(channel: &MqChannel) {
///     // Use the channel to publish or consume messages
/// }
/// ```
pub type MqChannel = Channel;

/// Establishes a connection to RabbitMQ and creates a channel.
///
/// # Parameters
/// - `uri`: The connection string for RabbitMQ (e.g., `amqp://root:example@localhost:5672/%2f`).
///
/// # Returns
/// - `Ok(Channel)`: A RabbitMQ `Channel` instance to perform operations.
/// - `Err(anyhow::Error)`: An error if the connection or channel creation fails.
///
/// # Example
/// ```no_run
/// # use anyhow;
/// use libs::messaging::get_channel;
///
/// # #[tokio::main]
/// # async fn main() -> anyhow::Result<()> {
/// let channel = get_channel("amqp://root:example@localhost:5672/%2f").await?;
/// # Ok(())
/// # }
/// ```
///
/// # Errors
/// - This function will return an error if:
///   - The URI is invalid.
///   - The connection to RabbitMQ fails.
///   - Creating a channel on the connection fails.
pub async fn get_channel(uri: &str) -> anyhow::Result<Channel> {
    let options = ConnectionProperties::default()
        .with_executor(tokio_executor_trait::Tokio::current())
        .with_reactor(tokio_reactor_trait::Tokio);

    let connection = Connection::connect(uri, options).await?;
    let channel = connection.create_channel().await?;

    Ok(channel)
}

/// Declares a queue in RabbitMQ using the provided channel.
///
/// # Parameters
/// - `channel`: The RabbitMQ `Channel` to declare the queue on.
/// - `queue_name`: The name of the queue to declare.
///
/// # Returns
/// - `Ok(())`: Indicates the queue was successfully declared.
/// - `Err(anyhow::Error)`: An error if the queue declaration fails.
///
/// # Example
/// ```no_run
/// # use anyhow;
/// use libs::messaging::{get_channel, init_mq};
///
/// # #[tokio::main]
/// # async fn main() -> anyhow::Result<()> {
/// let channel = get_channel("amqp://root:example@localhost:5672/%2f").await?;
/// init_mq(&channel, "my_queue").await?;
/// # Ok(())
/// # }
/// ```
///
/// # Errors
/// - This function will return an error if:
///   - The channel is invalid or closed.
///   - The queue declaration fails due to permissions or configuration issues.
pub async fn init_mq(channel: &Channel, queue_name: &str) -> anyhow::Result<()> {
    channel
        .queue_declare(
            queue_name,
            QueueDeclareOptions::default(),
            FieldTable::default(),
        )
        .await?;

    Ok(())
}

/// Creates a consumer for a given queue in RabbitMQ.
///
/// # Parameters
/// - `channel`: The RabbitMQ `Channel` to consume messages on.
/// - `queue_name`: The name of the queue to consume messages from.
/// - `consumer_tag`: A tag to identify the consumer.
///
/// # Returns
/// - `Ok(Consumer)`: A RabbitMQ `Consumer` instance to process messages.
/// - `Err(anyhow::Error)`: An error if the consumer creation fails.
///
/// # Example
/// ```no_run
/// # use anyhow;
/// use libs::messaging::{get_channel, create_consumer};
///
/// # #[tokio::main]
/// # async fn main() -> anyhow::Result<()> {
/// let channel = get_channel("amqp://root:example@localhost:5672/%2f").await?;
/// let consumer = create_consumer(&channel, "my_queue", "my_consumer").await?;
/// # Ok(())
/// # }
/// ```
///
/// # Errors
/// - This function will return an error if:
///   - The channel is invalid or closed.
///   - The queue does not exist or is inaccessible.
///   - The consumer creation fails due to configuration or permissions issues.
pub async fn create_consumer(
    channel: &Channel,
    queue_name: &str,
    consumer_tag: &str,
) -> anyhow::Result<Consumer> {
    let consumer = channel
        .basic_consume(
            queue_name,
            consumer_tag,
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .await?;
    Ok(consumer)
}

/// Publishes a job to a RabbitMQ queue.
///
/// # Parameters
/// - `channel`: The RabbitMQ `Channel` to publish the job on.
/// - `routing_key`: The routing key to determine the queue the message is sent to.
/// - `payload`: The payload of the job, which must implement `Serialize`.
///
/// # Returns
/// - `Ok(())`: Indicates the message was successfully published.
/// - `Err(anyhow::Error)`: An error if the message publication fails.
///
/// # Example
/// ```no_run
/// # use anyhow;
/// use serde::Serialize;
/// use libs::messaging::{get_channel, enqueue_job};
///
/// #[derive(Serialize)]
/// struct MyPayload {
///     data: String,
/// }
///
/// # #[tokio::main]
/// # async fn main() -> anyhow::Result<()> {
/// let channel = get_channel("amqp://root:example@localhost:5672/%2f").await?;
/// let payload = MyPayload { data: "Hello, world!".to_string() };
/// enqueue_job(&channel, "my_routing_key", &payload).await?;
/// # Ok(())
/// # }
/// ```
///
/// # Errors
/// - This function will return an error if:
///   - Serialization of the payload fails.
///   - The channel is invalid or closed.
///   - Publishing the message fails due to configuration or permissions issues.
pub async fn enqueue_job<T: Serialize>(
    channel: &Channel,
    routing_key: &str,
    payload: &T,
) -> anyhow::Result<()> {
    let payload_json = serde_json::to_vec(payload)?;
    channel
        .basic_publish(
            "",
            routing_key,
            BasicPublishOptions::default(),
            &payload_json,
            BasicProperties::default(),
        )
        .await?
        .await?;
    Ok(())
}
