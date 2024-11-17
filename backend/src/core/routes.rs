use super::{health_handler::health, team_handler::get_all_team_names};
use actix_web::web;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(health);
    cfg.service(get_all_team_names);
}
