use libs::caching;
use redis::{AsyncCommands, Client};
use std::{
    env,
    hash::{Hash, Hasher},
};

async fn cleanup(redis_client: Client, input_ids: Vec<u32>) {
    let mut connection = redis_client
        .get_multiplexed_tokio_connection()
        .await
        .unwrap();
    let cache_key = format!("cache:{}", {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        input_ids.iter().for_each(|id| id.hash(&mut hasher));
        hasher.finish().to_string()
    });
    let _: () = connection.del(cache_key).await.unwrap();
}

#[tokio::test]
async fn test_int_cache() -> anyhow::Result<()> {
    dotenv::dotenv().ok();

    let url = env::var("REDIS_URL").expect("REDIS_URL must be set in env");
    let redis_client = caching::init_redis(&url).await.unwrap();

    let input_ids = vec![1, 2, 3];
    let output_ids = vec![vec![10, 11, 12], vec![20, 21, 22]];

    caching::cache_result(&redis_client, input_ids.clone(), output_ids.clone())
        .await
        .unwrap();

    let retrieved_entry = caching::get_cached_result(&redis_client, &input_ids)
        .await
        .unwrap();

    assert!(retrieved_entry.is_some());
    let entry = retrieved_entry.unwrap();
    assert_eq!(entry.input_ids, input_ids);
    assert_eq!(entry.output_ids, output_ids);

    cleanup(redis_client, input_ids).await;

    Ok(())
}
