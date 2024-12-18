use std::sync::Arc;

use tokio::signal;

use best_combination_worker::{Processor, CONFIG};
use libs::{
    caching,
    constants::{DATABASE_NAME, STREAMING_PACKAGE_COLLECTION_NAME},
    db::{dao::StreamingPackageDao, DocumentDatabaseConnector, MongoClient},
    logging,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();

    logging::init_logging();

    let redis_client = caching::init_redis(&CONFIG.redis_url).await?;
    let mongo_client = MongoClient::init(&CONFIG.mongodb_uri, DATABASE_NAME).await;
    let package_dao =
        StreamingPackageDao::new(mongo_client.get_collection(STREAMING_PACKAGE_COLLECTION_NAME));

    let processor = Processor::new(Arc::new(redis_client), Arc::new(package_dao));
    processor.start().await?;

    let processor_handle = tokio::spawn(async move {
        if let Err(e) = processor.start().await {
            log::error!("Processor encountered an error: {:?}", e);
        }
    });

    signal::ctrl_c().await?;
    log::info!("Termination signal received. Shutting down...");

    // TODO: Gracefull shutdown

    processor_handle.abort();
    Ok(())
}
