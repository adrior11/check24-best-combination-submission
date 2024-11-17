use super::{health_handler, metrics_handler, team_handler};
use actix_web::web;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(health_handler::health);
    cfg.service(team_handler::get_all_team_names);
    cfg.service(metrics_handler::get_metrics);
}
