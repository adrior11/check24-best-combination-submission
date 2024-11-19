use once_cell::sync;

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

pub static REQUEST_DURATION: sync::Lazy<prometheus::HistogramVec> = sync::Lazy::new(|| {
    prometheus::HistogramVec::new(
        prometheus::HistogramOpts::new(
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

pub static REQUEST_COUNT: sync::Lazy<prometheus::IntCounterVec> = sync::Lazy::new(|| {
    prometheus::IntCounterVec::new(
        prometheus::Opts::new("api_requests_total", "Total number of API requests"),
        &[
            MetricLabelName::Method.as_str(),
            MetricLabelName::Endpoint.as_str(),
            MetricLabelName::Status.as_str(),
        ],
    )
    .expect("Failed to create REQUEST_COUNT")
});

pub static ERROR_COUNT: sync::Lazy<prometheus::IntCounter> = sync::Lazy::new(|| {
    prometheus::IntCounter::new("api_errors_total", "Total number of API errors")
        .expect("Failed to create ERROR_COUNT")
});
