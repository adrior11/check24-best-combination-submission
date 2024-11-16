use super::{get_collection_names, health};
use actix_web::web;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(health);
    cfg.service(get_collection_names);
}
