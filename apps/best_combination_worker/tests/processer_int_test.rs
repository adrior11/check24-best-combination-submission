use std::{env, sync::Arc};

use tokio::time::{sleep, Duration};

use best_combination_worker::{Processor, CONFIG};
use libs::{
    caching,
    constants::{DATABASE_NAME, STREAMING_PACKAGE_COLLECTION_NAME},
    db::{dao::StreamingPackageDao, DocumentDatabaseConnector, MongoClient},
    messaging::{self, init_mq},
    models::dtos::BestCombinationDto,
};

#[tokio::test]
async fn test_int_processor() {
    dotenv::dotenv().ok();

    let channel = messaging::get_channel(&CONFIG.rabbitmq_url).await.unwrap();
    init_mq(&channel, &CONFIG.task_queue_name).await.unwrap();
    init_mq(&channel, &CONFIG.result_queue_name).await.unwrap();

    let redis_client = caching::init_redis(&CONFIG.redis_url).await.unwrap();

    let uri = env::var("MONGODB_URI").expect("MONGODB_URI must be set in env");
    let mongo_client = MongoClient::init(&uri, DATABASE_NAME).await;
    let package_dao =
        StreamingPackageDao::new(mongo_client.get_collection(STREAMING_PACKAGE_COLLECTION_NAME));

    let processor = Processor::new(Arc::new(redis_client.clone()), Arc::new(package_dao));

    let processor_handle = tokio::spawn(async move {
        processor.start().await.unwrap();
    });

    sleep(Duration::from_secs(1)).await;

    let game_ids = vec![
        52, 69, 76, 79, 103, 89, 113, 121, 125, 139, 146, 151, 161, 171, 186, 193, 196, 212, 214,
        219, 225, 240, 251, 257, 261, 272, 284, 293, 307, 320, 302, 325, 337, 349, 356, 5305, 5320,
        5325, 5330, 5341, 5349, 5364, 5367, 5383, 5386, 5394, 5404, 5416, 5436, 5440, 5422, 5449,
        5459, 5467, 5474, 5483, 5492, 5501, 5511, 5525, 5529, 5541, 5548, 5557, 5566, 5584, 5573,
        5593, 7354, 7890, 8440, 8466, 8486, 8514, 8503, 8533, 8568, 8560, 8845,
    ];
    let test_message = serde_json::json!({
        "task_id": "test_task_123",
        "game_ids": game_ids,
        "limit": 3
    });

    messaging::enqueue_job(&channel, &CONFIG.task_queue_name, &test_message)
        .await
        .unwrap();

    sleep(Duration::from_secs(1)).await;

    let expected = [
        BestCombinationDto {
            packages: vec![4, 13, 37],
            combined_monthly_price_cents: 999,
            combined_monthly_price_yearly_subscription_in_cents: 699,
            coverage: 99,
        },
        BestCombinationDto {
            packages: vec![4, 13, 38],
            combined_monthly_price_cents: 2499,
            combined_monthly_price_yearly_subscription_in_cents: 1999,
            coverage: 99,
        },
        BestCombinationDto {
            packages: vec![4, 10, 13],
            combined_monthly_price_cents: 3599,
            combined_monthly_price_yearly_subscription_in_cents: 2999,
            coverage: 99,
        },
    ];

    let cached_result = caching::get_cached_result(&redis_client, &game_ids)
        .await
        .unwrap();
    assert!(cached_result.is_some());

    let cache_entry = cached_result.unwrap();
    assert_eq!(cache_entry.value, expected);

    processor_handle.abort();
}
