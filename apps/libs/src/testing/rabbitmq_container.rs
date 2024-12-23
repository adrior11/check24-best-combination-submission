use anyhow::Context;
use testcontainers::{core::IntoContainerPort, runners::AsyncRunner, GenericImage, ImageExt};

const RABBITMQ_IMAGE: (&str, &str) = ("rabbitmq", "3-management");
const RABBITMQ_PORT: u16 = 5672;

pub async fn init_rabbitmq_container() -> anyhow::Result<String> {
    let rabbitmq_container = GenericImage::new(RABBITMQ_IMAGE.0, RABBITMQ_IMAGE.1)
        .with_exposed_port(RABBITMQ_PORT.tcp())
        .with_env_var("RABBITMQ_DEFAULT_USER", "root")
        .with_env_var("RABBITMQ_DEFAULT_PASS", "example")
        .start()
        .await
        .context("Failed to start RabbitMQ testcontainer")?;

    let host = rabbitmq_container.get_host().await?;
    let uri = format!("amqp://root:example@{host}:{RABBITMQ_PORT}/%2f");

    Ok(uri)
}

#[cfg(test)]
mod tests {
    use lapin::{message::DeliveryResult, options::BasicAckOptions};

    use super::*;
    use crate::messaging;

    #[tokio::test]
    async fn test_rabbitmq_container_setup() {
        let queue_name = "TEST";
        let uri = init_rabbitmq_container().await.unwrap();
        let channel = messaging::get_channel(&uri).await.unwrap();
        messaging::init_mq(&channel, queue_name).await.unwrap();

        let consumer = messaging::create_consumer(&channel, queue_name, "TEST_CONSUMER")
            .await
            .unwrap();

        consumer.set_delegate(move |delivery: DeliveryResult| async move {
            assert!(delivery.is_ok());

            delivery
                .unwrap()
                .unwrap()
                .ack(BasicAckOptions::default())
                .await
                .unwrap();
        });

        let payload = "Hello World!".to_string();

        messaging::enqueue_job(&channel, queue_name, &payload)
            .await
            .unwrap();
    }
}
