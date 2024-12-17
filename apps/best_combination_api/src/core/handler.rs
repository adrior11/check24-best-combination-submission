use actix_web::{web::Data, HttpResponse};
use async_graphql::http::{self, GraphQLPlaygroundConfig};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};

use super::resolver::AppSchema;

pub async fn index(schema: Data<AppSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

pub async fn index_playground() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(http::playground_source(GraphQLPlaygroundConfig::new("/")))
}
