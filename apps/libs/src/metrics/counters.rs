// TODO: Add a metric regarding teams size of input
use once_cell::sync::Lazy;
use prometheus::{HistogramOpts, HistogramVec, IntCounter, IntCounterVec, Opts};

enum MetricLabelName {
    Method,
    Endpoint,
    Status,
}

impl MetricLabelName {
    fn as_str(&self) -> &'static str {
        match self {
            MetricLabelName::Method => "method",
            MetricLabelName::Endpoint => "endpoint",
            MetricLabelName::Status => "status",
        }
    }
}

pub static REQUEST_DURATION: Lazy<HistogramVec> = Lazy::new(|| {
    HistogramVec::new(
        HistogramOpts::new(
            "api_request_duration_seconds",
            "Histogram for the duration of API requests in seconds",
        ),
        &[
            MetricLabelName::Method.as_str(),
            MetricLabelName::Endpoint.as_str(),
            MetricLabelName::Status.as_str(),
        ],
    )
    .expect("Failed to create REQUEST_DURATION")
});

pub static REQUEST_COUNT: Lazy<IntCounterVec> = Lazy::new(|| {
    IntCounterVec::new(
        Opts::new("api_requests_total", "Total number of API requests"),
        &[
            MetricLabelName::Method.as_str(),
            MetricLabelName::Endpoint.as_str(),
            MetricLabelName::Status.as_str(),
        ],
    )
    .expect("Failed to create REQUEST_COUNT")
});

pub static ERROR_COUNT: Lazy<IntCounter> = Lazy::new(|| {
    IntCounter::new("api_errors_total", "Total number of API errors")
        .expect("Failed to create ERROR_COUNT")
});

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use validator::ValidateContains;

    // Utility function to clear metrics state
    fn reset_metrics() {
        REQUEST_COUNT.reset();
        REQUEST_DURATION.reset();
        ERROR_COUNT.reset();
    }

    #[test]
    fn test_metrics_initialization() {
        reset_metrics();

        assert!(prometheus::default_registry()
            .register(Box::new(REQUEST_DURATION.clone()))
            .is_ok());
        assert!(prometheus::default_registry()
            .register(Box::new(REQUEST_COUNT.clone()))
            .is_ok());
        assert!(prometheus::default_registry()
            .register(Box::new(ERROR_COUNT.clone()))
            .is_ok());
    }

    #[test]
    fn test_request_count_increment() {
        reset_metrics();

        REQUEST_COUNT
            .with_label_values(&["GET", "/test", "200"])
            .inc();
        REQUEST_COUNT
            .with_label_values(&["GET", "/test", "200"])
            .inc_by(2);

        let metric_families = prometheus::default_registry().gather();
        for mf in metric_families {
            if mf.get_name() == "api_requests_total" {
                for m in mf.get_metric() {
                    let labels = m.get_label();
                    let mut label_map = HashMap::new();
                    for label in labels {
                        label_map.insert(label.get_name(), label.get_value());
                    }
                    if label_map.get("method").validate_contains("GET")
                        && label_map.get("endpoint").validate_contains("/test")
                        && label_map.get("status").validate_contains("200")
                    {
                        assert_eq!(m.get_counter().get_value(), 3.0);
                    }
                }
            }
        }
    }

    #[test]
    fn test_error_count_increment() {
        reset_metrics();

        ERROR_COUNT.inc();
        ERROR_COUNT.inc_by(4);

        let metric_families = prometheus::default_registry().gather();
        for mf in metric_families {
            if mf.get_name() == "api_errors_total" {
                for m in mf.get_metric() {
                    assert_eq!(m.get_counter().get_value(), 5.0);
                }
            }
        }
    }

    #[test]
    fn test_request_duration_observe() {
        reset_metrics();

        REQUEST_DURATION
            .with_label_values(&["POST", "/submit", "201"])
            .observe(0.123);

        let metric_families = prometheus::default_registry().gather();
        for mf in metric_families {
            if mf.get_name() == "api_request_duration_seconds" {
                for m in mf.get_metric() {
                    let labels = m.get_label();
                    let mut label_map = HashMap::new();
                    for label in labels {
                        label_map.insert(label.get_name(), label.get_value());
                    }
                    if label_map.get("method").validate_contains("POST")
                        && label_map.get("endpoint").validate_contains("/submit")
                        && label_map.get("status").validate_contains("201")
                    {
                        let histogram = m.get_histogram();
                        assert_eq!(histogram.get_sample_count(), 1);
                        assert_eq!(histogram.get_sample_sum(), 0.123);
                    }
                }
            }
        }
    }
}
