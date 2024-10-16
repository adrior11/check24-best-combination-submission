mod macros;
mod model;
mod db_utils;

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
#[allow(unused_imports)]
use mongodb::{bson::doc, options::IndexOptions, Client, Collection, IndexModel};

#[allow(dead_code)]
struct AppState {
    mongo_client: Client,
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/health")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("Backend is healthy!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let uri = std::env::var("MONGO_DB_URI")
        .expect("MONGO_DB_URI must be set in .env");

    let client = Client::with_uri_str(uri).await
        .expect("Failed to connect");

    db_utils::populate_db(&client).await.expect("Failed to populate the database");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState {
                mongo_client: client.clone(),
            }))
            .service(hello)
            .service(health_check)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
