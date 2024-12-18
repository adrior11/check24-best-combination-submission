use std::{collections::BTreeSet, env, time::Duration};

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use libs::{
    constants::{DATABASE_NAME, STREAMING_PACKAGE_COLLECTION_NAME},
    db::{dao::StreamingPackageDao, DocumentDatabaseConnector, MongoClient},
    models::dtos::BestCombinationSubsetDto,
};

async fn init_mongo_client() -> MongoClient {
    dotenv::dotenv().ok();
    let uri = env::var("MONGODB_URI").unwrap();
    MongoClient::init(&uri, DATABASE_NAME).await
}

async fn preprocess_aggregation(
    package_dao: &StreamingPackageDao,
    ids: &BTreeSet<usize>,
) -> Vec<BestCombinationSubsetDto> {
    package_dao.preprocess_subsets(ids).await.unwrap()
}

fn bench_mongo_preprocessing(c: &mut Criterion) {
    let mut group = c.benchmark_group("mongo_preprocessing");
    group.measurement_time(Duration::from_secs(40));
    group.sample_size(30);

    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    let package_dao = rt.block_on(async {
        let mongo_client = init_mongo_client().await;
        StreamingPackageDao::new(mongo_client.get_collection(STREAMING_PACKAGE_COLLECTION_NAME))
    });
    let ids: BTreeSet<usize> = (1..=8876).collect();

    group.bench_function("aggregation", |b| {
        b.to_async(&rt)
            .iter(|| preprocess_aggregation(black_box(&package_dao), black_box(&ids)));
    });

    group.finish();
}

criterion_group!(benches, bench_mongo_preprocessing);
criterion_main!(benches);
