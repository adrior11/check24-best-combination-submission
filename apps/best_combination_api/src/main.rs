use std::io;

use actix_web::{web, App, HttpServer};
use async_graphql::{EmptyMutation, EmptySubscription, Schema};

use best_combination_api::{QueryRoot, CONFIG};
use libs::{
    caching,
    constants::{DATABASE_NAME, GAME_COLLECTION_NAME},
    db::{dao::GameDao, DocumentDatabaseConnector, MongoClient},
    logging, messaging,
    metrics::MetricsMiddleware,
};

#[actix_web::main]
async fn main() -> io::Result<()> {
    dotenv::dotenv().ok();

    logging::init_logging();

    let mq_channel = messaging::get_channel(&CONFIG.rabbitmq_url).await.unwrap();
    let redis_client = caching::init_redis(&CONFIG.redis_url).await.unwrap();
    let mongo_client = MongoClient::init(&CONFIG.mongodb_uri, DATABASE_NAME).await;
    let game_dao = GameDao::new(mongo_client.get_collection(GAME_COLLECTION_NAME));

    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
        .data(mq_channel.clone())
        .data(redis_client.clone())
        .data(game_dao)
        .finish();

    HttpServer::new(move || {
        App::new()
            .service(
                web::resource("/")
                    .route(web::post().to(best_combination_api::index))
                    .route(web::post().to(best_combination_api::index_playground)),
            )
            .app_data(schema.clone())
            .wrap(logging::request_logger())
            .wrap(MetricsMiddleware)
    })
    .bind("0.0.0.0:8000")?
    .run()
    .await
}
