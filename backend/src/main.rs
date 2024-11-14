use actix_web::web;
use mongodb::bson;
use std::{env, io};

#[allow(dead_code)]
struct AppState {
    mongo_client: mongodb::Client,
}

#[actix_web::get("/")]
async fn hello() -> impl actix_web::Responder {
    actix_web::HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    println!("Setting up backend...");

    dotenv::dotenv().ok();

    println!("Reading .env variable...");

    let uri = env::var("MONGO_DB_URI").expect("MONGO_DB_URI must be set in .env");

    println!("Connecting to MongoDB...");

    let client = mongodb::Client::with_uri_str(uri)
        .await
        .expect("Failed to connect to MongoDB");

    client
        .database("admin")
        .run_command(bson::doc! { "ping": 1 })
        .await
        .expect("Failed to reach MongoDB server");

    println!("Setting up server...");

    actix_web::HttpServer::new(move || {
        actix_web::App::new()
            .app_data(web::Data::new(AppState {
                mongo_client: client.clone(),
            }))
            .service(hello)
    })
    .bind(("0.0.0.0", 8000))? // NOTE: Use from .env
    .run()
    .await
}
