use std::sync::Arc;

use futures::stream::StreamExt;
use lapin::{message::Delivery, options::BasicAckOptions, Channel, Consumer};

use libs::{
    caching::{self, CacheValue, RedisClient},
    db::dao::StreamingPackageDao,
    messaging,
    models::payloads::TaskMessagePayload,
};

use super::service;
use crate::config::CONFIG;

pub struct Processor {
    redis_client: Arc<RedisClient>,
    package_dao: Arc<StreamingPackageDao>,
}

impl Clone for Processor {
    fn clone(&self) -> Self {
        Processor {
            redis_client: Arc::clone(&self.redis_client),
            package_dao: Arc::clone(&self.package_dao),
        }
    }
}

impl Processor {
    pub fn new(redis_client: Arc<RedisClient>, package_dao: Arc<StreamingPackageDao>) -> Self {
        Processor {
            redis_client,
            package_dao,
        }
    }

    pub async fn start(&self) -> anyhow::Result<()> {
        let channel = messaging::get_channel(&CONFIG.rabbitmq_url).await?;
        let consumer =
            messaging::create_consumer(&channel, &CONFIG.task_queue_name, "rust_processor").await?;

        log::info!("Processor is running an waiting for messages...");

        self.handle_messages(channel, consumer).await?;

        Ok(())
    }

    async fn handle_messages(
        &self,
        channel: Channel,
        mut consumer: Consumer,
    ) -> anyhow::Result<()> {
        while let Some(delivery) = consumer.next().await {
            match delivery {
                Ok(delivery) => {
                    let processor = self.clone();
                    let channel = channel.clone();
                    tokio::spawn(async move {
                        if let Err(e) = processor.process_message(&channel, &delivery).await {
                            log::error!("Failed to process message: {:?}", e);
                            // TODO: Nack to message / attempt retries
                        }
                    });
                }
                Err(e) => {
                    log::error!("Error in consumer stream: {:?}", e);
                }
            }
        }
        Ok(())
    }

    async fn process_message(&self, channel: &Channel, delivery: &Delivery) -> anyhow::Result<()> {
        let msg = self.parse_message(&delivery.data)?;
        let subsets = self.package_dao.preprocess_subsets(&msg.game_ids).await?;

        log::info!("Performing best combination set cover algorithm...");
        let best_combinations = service::get_best_combination(&msg.game_ids, &subsets, msg.limit);

        let cache_key: Vec<usize> = msg.game_ids.into_iter().collect();
        caching::cache_entry(
            &self.redis_client,
            cache_key,
            CacheValue::Data(best_combinations),
        )
        .await?;

        channel
            .basic_ack(delivery.delivery_tag, BasicAckOptions::default())
            .await?;

        log::info!("Finished processing message");

        Ok(())
    }

    fn parse_message(&self, data: &[u8]) -> anyhow::Result<TaskMessagePayload> {
        let msg: TaskMessagePayload = serde_json::from_slice(data)?;
        log::info!("Received job: {:?}", msg);
        Ok(msg)
    }

    pub async fn abort() {
        todo!("Graceful shutdown");
    }
}
