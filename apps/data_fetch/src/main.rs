use std::{io, sync::Arc};

use actix_web::{
    web::{self, Data},
    App, HttpServer,
};
use async_graphql::{EmptyMutation, EmptySubscription, Schema};

use data_fetch::{Query, CONFIG};
use libs::{
    constants::{DATABASE_NAME, GAME_COLLECTION_NAME},
    db::{dao::GameDao, DocumentDatabaseConnector, MongoClient},
    logging,
    metrics::{self, MetricsMiddleware},
};

#[actix_web::main]
async fn main() -> io::Result<()> {
    dotenv::dotenv().ok();

    logging::init_logging();

    let mongo_client = MongoClient::init(&CONFIG.mongodb_uri, DATABASE_NAME).await;
    let game_dao = GameDao::new(mongo_client.get_collection(GAME_COLLECTION_NAME));

    let schema = Schema::build(Query, EmptyMutation, EmptySubscription)
        .data(Arc::new(game_dao))
        .enable_federation()
        .finish();

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(schema.clone()))
            .app_data(Data::new(metrics::init_metrics()))
            .service(
                web::resource("/")
                    .route(web::post().to(data_fetch::index))
                    .route(web::get().to(data_fetch::index_playground)),
            )
            .route("/metrics", web::get().to(metrics::metrics_handler))
            .wrap(logging::request_logger())
            .wrap(MetricsMiddleware)
    })
    .bind(format!("0.0.0.0:{}", &CONFIG.data_fetch_service_port))?
    .run()
    .await
}
