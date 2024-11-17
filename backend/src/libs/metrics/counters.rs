use once_cell::sync;

pub static REQUEST_DURATION: sync::Lazy<prometheus::HistogramVec> = sync::Lazy::new(|| {
    prometheus::HistogramVec::new(
        prometheus::HistogramOpts::new(
            "api_request_duration_seconds",
            "Histogram for the duration of API requests in seconds",
        ),
        &["method", "endpoint", "status"],
    )
    .expect("Failed to create REQUEST_DURATION")
});

pub static REQUEST_COUNT: sync::Lazy<prometheus::IntCounterVec> = sync::Lazy::new(|| {
    prometheus::IntCounterVec::new(
        prometheus::Opts::new("api_requests_total", "Total number of API requests"),
        &["method", "endpoint", "status"],
    )
    .expect("Failed to create REQUEST_COUNT")
});

pub static ERROR_COUNT: sync::Lazy<prometheus::IntCounter> = sync::Lazy::new(|| {
    prometheus::IntCounter::new("api_errors_total", "Total number of API errors")
        .expect("Failed to create ERROR_COUNT")
});
