mod app;
mod common;
mod config;
mod core;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    app::run().await
}
