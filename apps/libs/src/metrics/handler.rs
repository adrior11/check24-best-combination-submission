use actix_web::web::Data;
use prometheus::Registry;

use super::registry::gather_metrics;

/// HTTP handler for serving Prometheus metrics.
///
/// This handler gathers all registered metrics from the provided Prometheus `Registry`
/// and returns them in a Prometheus-compatible text format. The `/metrics` endpoint
/// is typically scraped by Prometheus for monitoring.
///
/// # Arguments
///
/// * `data` - A shared reference to the Prometheus `Registry`, injected via Actix's `Data` extractor.
///
/// # Returns
///
/// An HTTP response containing the gathered metrics in text format.
///
pub async fn metrics_handler(data: Data<Registry>) -> impl actix_web::Responder {
    let registry = data.get_ref();
    gather_metrics(registry)
}
