// NOTE: Registry is not needed
use super::counters;
use prometheus::{Encoder, Registry, TextEncoder};

/// Registers all defined Prometheus metrics with the provided registry.
///
/// # Arguments
///
/// * `registry` - A mutable reference to a Prometheus `Registry` where metrics will be registered.
///
/// # Panics
///
/// This function will panic if any metric fails to register.
fn register_metrics(registry: &Registry) {
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
pub fn init_metrics() -> Registry {
    let registry = Registry::new();
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
pub fn gather_metrics(registry: &Registry) -> String {
    let metric_families = registry.gather();
    let encoder = TextEncoder::new();
    let mut buffer = Vec::new();

    encoder
        .encode(&metric_families, &mut buffer)
        .expect("Failed to encode metrics");

    String::from_utf8(buffer).expect("Failed to convert metrics to UTF-8")
}

#[cfg(test)]
mod tests {
    use super::*;
    use prometheus::Registry;

    #[test]
    fn test_register_metrics_success() {
        let registry = Registry::new();
        register_metrics(&registry);

        let metric_families = registry.gather();
        let metric_names: Vec<_> = metric_families.iter().map(|mf| mf.get_name()).collect();

        assert!(metric_names.contains(&"api_request_duration_seconds"));
        assert!(metric_names.contains(&"api_requests_total"));
        assert!(metric_names.contains(&"api_errors_total"));
    }

    #[test]
    fn test_init_metrics_creates_registry() {
        let registry = init_metrics();

        let metric_families = registry.gather();
        let metric_names: Vec<_> = metric_families.iter().map(|mf| mf.get_name()).collect();

        assert!(metric_names.contains(&"api_request_duration_seconds"));
        assert!(metric_names.contains(&"api_requests_total"));
        assert!(metric_names.contains(&"api_errors_total"));
    }

    #[test]
    fn test_gather_metrics_encodes_metrics() {
        let registry = init_metrics();

        counters::REQUEST_COUNT
            .with_label_values(&["GET", "/test", "200"])
            .inc();
        counters::REQUEST_DURATION
            .with_label_values(&["GET", "/test", "200"])
            .observe(0.123);
        counters::ERROR_COUNT.inc();

        let encoded_metrics = gather_metrics(&registry);

        assert!(encoded_metrics.contains("api_request_duration_seconds"));
        assert!(encoded_metrics.contains("api_requests_total"));
        assert!(encoded_metrics.contains("api_errors_total"));
        assert!(encoded_metrics.contains("0.123"));
        assert!(encoded_metrics.contains("1"));
    }
}
