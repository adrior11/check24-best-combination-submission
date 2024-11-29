use crate::{
    api::{resolvers::QueryRoot, schema::AppSchema},
    core,
    libs::{
        caching, logging,
        metrics::{self, middleware::MetricsMiddleware},
        mongo,
    },
};
use actix_web::{
    web::{self, Data},
    App, HttpResponse, HttpServer,
};
use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use prometheus::Registry;
use std::{io, sync::Arc};

#[allow(dead_code)]
pub struct AppState {
    pub mongo_client: Arc<mongodb::Client>,
    pub redis_client: Arc<redis::Client>,
    pub registry: Arc<Registry>,
}

pub async fn run() -> io::Result<()> {
    dotenv::dotenv().ok();

    logging::init_logging();

    let mongo_client = mongo::init_mongodb().await.unwrap();

    let redis_client = caching::init_redis().await.unwrap();

    let registry = metrics::init_metrics();

    let app_state = Arc::new(AppState {
        mongo_client: Arc::new(mongo_client),
        redis_client: Arc::new(redis_client),
        registry: Arc::new(registry),
    });

    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
        .data(app_state.clone())
        .finish();

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(app_state.clone()))
            .app_data(Data::new(schema.clone()))
            .service(
                web::resource("/graphql")
                    .route(web::post().to(graphql_handler))
                    .route(web::get().to(graphql_playground)),
            )
            .configure(core::configure_routes)
            .wrap(MetricsMiddleware)
            .wrap(logging::request_logger())
    })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}

async fn graphql_handler(schema: Data<AppSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn graphql_playground() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(async_graphql::http::playground_source(
            async_graphql::http::GraphQLPlaygroundConfig::new("/graphql"),
        ))
}
