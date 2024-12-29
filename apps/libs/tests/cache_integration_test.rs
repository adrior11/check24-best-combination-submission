use std::collections::HashMap;

use libs::{
    caching::{self, CacheValue, CompositeKey},
    models::{
        dtos::{BestCombinationDto, BestCombinationPackageDto},
        fetch_types::FetchOptions,
    },
    testing,
};

#[ignore = "CI needs testcontainer configuration in shell"]
#[tokio::test]
async fn test_int_cache() -> anyhow::Result<()> {
    dotenv::dotenv().ok();

    let url = testing::init_redis_container().await.unwrap();
    let redis_client = caching::init_redis(&url).await.unwrap();

    let key = CompositeKey::new(vec![1, 2, 3], FetchOptions::new(1));
    let value = vec![BestCombinationDto {
        packages: vec![
            BestCombinationPackageDto {
                id: 4,
                coverage: HashMap::new(),
                monthly_price_cents: Some(10),
                monthly_price_yearly_subscription_in_cents: 10,
            },
            BestCombinationPackageDto {
                id: 13,
                coverage: HashMap::new(),
                monthly_price_cents: Some(10),
                monthly_price_yearly_subscription_in_cents: 10,
            },
            BestCombinationPackageDto {
                id: 37,
                coverage: HashMap::new(),
                monthly_price_cents: Some(10),
                monthly_price_yearly_subscription_in_cents: 10,
            },
        ],
        combined_monthly_price_cents: 30,
        combined_monthly_price_yearly_subscription_in_cents: 30,
        combined_coverage: 99,
    }];

    caching::cache_entry(&redis_client, &key, CacheValue::Data(value.clone()))
        .await
        .unwrap();

    let retrieved_entry =
        caching::get_cached_entry::<CompositeKey, Vec<BestCombinationDto>>(&redis_client, &key)
            .await
            .unwrap();

    assert!(retrieved_entry.is_some());
    let entry = retrieved_entry.unwrap();
    assert_eq!(entry.key, key);
    assert_eq!(entry.value, CacheValue::Data(value));

    Ok(())
}
