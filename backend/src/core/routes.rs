use super::{health::health_handler, metrics::metrics_handler, team::team_handler};
use actix_web::web;

// TODO: Transition to GraphQL
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(health_handler::health);
    cfg.service(team_handler::get_all_team_names);
    cfg.service(metrics_handler::get_metrics);
}
