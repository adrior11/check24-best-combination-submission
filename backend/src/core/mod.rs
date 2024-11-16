mod best_combination_handler;
mod collection_handler;
mod collection_service;
mod health_handler;
pub mod models;
mod routes;
mod tests;

use collection_handler::get_all_collection_names;
use health_handler::health;
pub use routes::configure_routes;
