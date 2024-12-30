use std::{io, sync::Arc};

use actix_cors::Cors;
use actix_web::{
    web::{self, Data},
    App, HttpServer,
};
use async_graphql::{EmptySubscription, Schema};

use best_combination_api::{Mutation, Query, CONFIG};
use libs::{
    caching,
    constants::{DATABASE_NAME, GAME_COLLECTION_NAME},
    db::{dao::GameDao, DocumentDatabaseConnector, MongoClient},
    logging, messaging,
    metrics::{self, MetricsMiddleware},
};

#[actix_web::main]
async fn main() -> io::Result<()> {
    dotenv::dotenv().ok();

    logging::init_logging();

    let mq_channel = messaging::get_channel(&CONFIG.rabbitmq_url).await.unwrap();
    messaging::init_mq(&mq_channel, &CONFIG.task_queue_name)
        .await
        .unwrap();

    let redis_client = caching::init_redis(&CONFIG.redis_url).await.unwrap();

    let mongo_client = MongoClient::init(&CONFIG.mongodb_uri, DATABASE_NAME).await;
    let game_dao = GameDao::new(mongo_client.get_collection(GAME_COLLECTION_NAME));

    let schema = Schema::build(Query, Mutation, EmptySubscription)
        .data(Arc::new(mq_channel.clone()))
        .data(Arc::new(redis_client.clone()))
        .data(Arc::new(game_dao))
        .enable_federation()
        .finish();

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(schema.clone()))
            .app_data(Data::new(metrics::init_metrics()))
            .service(
                web::resource("/")
                    .route(web::post().to(best_combination_api::index))
                    .route(web::get().to(best_combination_api::index_playground)),
            )
            .route("/metrics", web::get().to(metrics::metrics_handler))
            .wrap(logging::request_logger())
            .wrap(MetricsMiddleware)
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header(),
            )
    })
    .bind(format!("0.0.0.0:{}", &CONFIG.api_service_port))?
    .run()
    .await

    // FIX: Graceful shutdown of lapin (RabbitMQ) via tokio
}
