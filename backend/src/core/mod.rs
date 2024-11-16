mod collection_handler;
mod collection_service;
mod health_handler;
mod routes;

use collection_handler::get_collection_names;
use health_handler::health;
pub use routes::configure_routes;
