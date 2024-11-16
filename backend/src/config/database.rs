use mongodb::{bson, error};
use std::env;

pub async fn init_mongodb() -> error::Result<mongodb::Client> {
    dotenv::dotenv().ok();

    let uri = env::var("MONGODB_URI").expect("MONGODB_URI must be set in .env");

    let client = mongodb::Client::with_uri_str(uri)
        .await
        .expect("Failed to connect to MongoDB");

    client
        .database("admin")
        .run_command(bson::doc! { "ping": 1 })
        .await
        .expect("Failed to reach MongoDB server");

    Ok(client)
}
