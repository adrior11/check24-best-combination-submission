use std::{env, sync::Arc};

use best_combination_worker::{Processor, CONFIG};
use libs::{
    caching::{self, CacheValue},
    constants::{DATABASE_NAME, STREAMING_PACKAGE_COLLECTION_NAME},
    db::{dao::StreamingPackageDao, DocumentDatabaseConnector, MongoClient},
    messaging::{self, init_mq},
    models::dtos::{BestCombinationDto, BestCombinationPackageDto},
    testing,
};

#[tokio::test]
async fn test_int_processor() {
    dotenv::dotenv().ok();

    let rabbitmq_url = testing::init_rabbitmq_container().await.unwrap();
    let channel = messaging::get_channel(&rabbitmq_url).await.unwrap();
    init_mq(&channel, &CONFIG.task_queue_name).await.unwrap();

    let redis_url = testing::init_redis_container().await.unwrap();
    let redis_client = caching::init_redis(&redis_url).await.unwrap();

    let uri = env::var("MONGODB_URI").expect("MONGODB_URI must be set in env");
    let mongo_client = MongoClient::init(&uri, DATABASE_NAME).await;
    let package_dao =
        StreamingPackageDao::new(mongo_client.get_collection(STREAMING_PACKAGE_COLLECTION_NAME));

    let processor = Processor::new(Arc::new(redis_client.clone()), Arc::new(package_dao));

    let processor_handle = tokio::spawn(async move {
        processor.start().await.unwrap();
    });

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

    let expected = [
        BestCombinationDto::new(
            vec![
                BestCombinationPackageDto::new(
                    3,
                    vec![
                        ("UEFA Champions League 24/25", (0, 2)),
                        ("Bundesliga 24/25", (0, 2)),
                        ("Bundesliga 23/24", (0, 2)),
                    ],
                    Some(0),
                    0,
                ),
                BestCombinationPackageDto::new(
                    37,
                    vec![
                        ("UEFA Champions League 24/25", (0, 2)),
                        ("DFB Pokal 24/25", (0, 2)),
                    ],
                    Some(999),
                    699,
                ),
            ],
            999,
            699,
            99,
        ),
        BestCombinationDto::new(
            vec![
                BestCombinationPackageDto::new(
                    3,
                    vec![
                        ("UEFA Champions League 24/25", (0, 2)),
                        ("Bundesliga 24/25", (0, 2)),
                        ("Bundesliga 23/24", (0, 2)),
                    ],
                    Some(0),
                    0,
                ),
                BestCombinationPackageDto::new(
                    38,
                    vec![
                        ("UEFA Champions League 24/25", (0, 2)),
                        ("DFB Pokal 24/25", (0, 2)),
                    ],
                    Some(2499),
                    1999,
                ),
            ],
            2499,
            1999,
            99,
        ),
        BestCombinationDto::new(
            vec![
                BestCombinationPackageDto::new(
                    3,
                    vec![
                        ("UEFA Champions League 24/25", (0, 2)),
                        ("Bundesliga 24/25", (0, 2)),
                        ("Bundesliga 23/24", (0, 2)),
                    ],
                    Some(0),
                    0,
                ),
                BestCombinationPackageDto::new(
                    10,
                    vec![("Bundesliga 24/25", (1, 2)), ("DFB Pokal 24/25", (2, 2))],
                    Some(3599),
                    2999,
                ),
            ],
            3599,
            2999,
            99,
        ),
    ];

    let cached_result = caching::get_cached_entry(&redis_client, &game_ids)
        .await
        .unwrap();
    assert!(cached_result.is_some());

    let cache_entry = cached_result.unwrap();
    assert_eq!(cache_entry.value, CacheValue::Data(expected));

    processor_handle.abort();
}
