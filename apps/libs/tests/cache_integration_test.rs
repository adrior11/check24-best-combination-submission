use std::{
    env,
    hash::{Hash, Hasher},
};

use redis::{AsyncCommands, Client};

use libs::{caching, models::dtos::BestCombinationDto};

async fn cleanup(redis_client: Client, input_ids: Vec<usize>) {
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

    let key = vec![1, 2, 3];
    let value = vec![
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
    ];

    caching::cache_result(&redis_client, key.clone(), value.clone())
        .await
        .unwrap();

    let retrieved_entry = caching::get_cached_result(&redis_client, &key)
        .await
        .unwrap();

    assert!(retrieved_entry.is_some());
    let entry = retrieved_entry.unwrap();
    assert_eq!(entry.key, key);
    assert_eq!(entry.value, value);

    cleanup(redis_client, key).await;

    Ok(())
}
