use super::handlers;
use actix_web::web;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(handlers::hello);
    cfg.service(handlers::health);
}
