use super::counters;
use prometheus::Encoder;

/// Registers all defined Prometheus metrics with the provided registry.
///
/// # Arguments
///
/// * `registry` - A mutable reference to a Prometheus `Registry` where metrics will be registered.
///
/// # Panics
///
/// This function will panic if any metric fails to register.
fn register_metrics(registry: &prometheus::Registry) {
    registry
        .register(Box::new(counters::REQUEST_DURATION.clone()))
        .expect("Failed to register REQUEST_DURATION");

    registry
        .register(Box::new(counters::REQUEST_COUNT.clone()))
        .expect("Failed to register REQUEST_COUNT");

    registry
        .register(Box::new(counters::ERROR_COUNT.clone()))
        .expect("Failed to register ERROR_COUNT");
}

/// Initializes the Prometheus metrics system.
///
/// Creates a new Prometheus `Registry`, registers all metrics, and returns the registry.
/// This function is intended to be called once during application startup.
///
/// # Returns
///
/// A `Registry` instance with all metrics registered.
pub fn init_metrics() -> prometheus::Registry {
    let registry = prometheus::Registry::new();
    register_metrics(&registry);
    registry
}

/// Gathers all registered metrics and encodes them in Prometheus-compatible text format.
///
/// # Returns
///
/// A `String` containing the encoded metrics data, ready to be served via an HTTP endpoint.
///
/// # Panics
///
/// This function will panic if encoding or UTF-8 conversion fails.
pub fn gather_metrics(registry: &prometheus::Registry) -> String {
    let metric_families = registry.gather();
    let encoder = prometheus::TextEncoder::new();
    let mut buffer = Vec::new();

    encoder
        .encode(&metric_families, &mut buffer)
        .expect("Failed to encode metrics");

    String::from_utf8(buffer).expect("Failed to convert metrics to UTF-8")
}
