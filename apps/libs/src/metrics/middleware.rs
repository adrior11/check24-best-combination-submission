use std::{
    future::{self, Future, Ready},
    pin::Pin,
    task::{Context, Poll},
    time::Instant,
};

use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    http::StatusCode,
    Error,
};

use super::counters::{ERROR_COUNT, REQUEST_COUNT, REQUEST_DURATION};

/// Middleware for collecting Prometheus metrics on HTTP requests.
///
/// Tracks:
/// - Request count (`REQUEST_COUNT`)
/// - Request duration (`REQUEST_DURATION`)
/// - Errors (`ERROR_COUNT`)
///
/// Excludes tracking for specific routes like `/metrics`.
pub struct MetricsMiddleware;

impl<S, B> Transform<S, ServiceRequest> for MetricsMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = MetricsMiddlewareImpl<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    /// Creates a new transformation layer for the middleware.
    fn new_transform(&self, service: S) -> Self::Future {
        future::ready(Ok(MetricsMiddlewareImpl { service }))
    }
}

/// Inner implementation of the MetricsMiddleware.
pub struct MetricsMiddlewareImpl<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for MetricsMiddlewareImpl<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    /// Polls the readiness of the service.
    fn poll_ready(&self, ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    /// Handles an incoming HTTP request and collects metrics.
    fn call(&self, req: ServiceRequest) -> Self::Future {
        let method = req.method().to_string();
        let path = req.path().to_string();

        log::debug!("Incoming request: method={}, path={}", method, path);

        // Skip metrics collection for the /metrics endpoint.
        if path == "/metrics" {
            return Box::pin(self.service.call(req));
        }

        let start_time = Instant::now();
        let fut = self.service.call(req);

        Box::pin(async move {
            let response = fut.await;
            let elapsed = start_time.elapsed().as_secs_f64();

            let status = match &response {
                Ok(res) => res.response().status(),
                Err(_) => {
                    log::debug!("Request failed: method={}, path={}", method, path);
                    StatusCode::INTERNAL_SERVER_ERROR
                }
            };

            // Increament ERROR_COUNT if status code is 5xx
            if status.is_server_error() {
                ERROR_COUNT.inc();
                log::error!(
                    "Request failed: method={}, path={}, status={}",
                    method,
                    path,
                    status
                );
            }

            let status_code = status.as_u16().to_string();

            // Update request count and duration metrics
            REQUEST_COUNT
                .with_label_values(&[&method, &path, &status_code])
                .inc();
            REQUEST_DURATION
                .with_label_values(&[&method, &path, &status_code])
                .observe(elapsed);

            response
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{
        http::header::ContentType,
        http::StatusCode,
        test::{self, TestRequest},
        App, HttpResponse, Responder,
    };

    // Utility function to clear metrics state
    fn reset_metrics() {
        REQUEST_COUNT.reset();
        REQUEST_DURATION.reset();
        ERROR_COUNT.reset();
    }

    #[actix_web::get("/test")]
    async fn mock_handler() -> impl Responder {
        actix_web::HttpResponse::Ok()
            .content_type(ContentType::plaintext())
            .body("Hello World!")
    }

    #[actix_web::get("/error")]
    async fn error_mock_handler() -> impl Responder {
        HttpResponse::InternalServerError()
            .content_type(ContentType::plaintext())
            .body("Error")
    }

    #[actix_web::test]
    async fn test_metrics_middleware() {
        let app = test::init_service(
            App::new()
                .wrap(MetricsMiddleware)
                .service(mock_handler)
                .service(error_mock_handler),
        )
        .await;

        {
            reset_metrics();

            let req = TestRequest::get().uri("/test").to_request();
            let resp = test::call_service(&app, req).await;

            assert_eq!(resp.status(), StatusCode::OK);

            let request_count = REQUEST_COUNT
                .with_label_values(&["GET", "/test", "200"])
                .get();
            assert_eq!(request_count, 1);

            let histogram = REQUEST_DURATION.with_label_values(&["GET", "/test", "200"]);
            let metrics = histogram.get_sample_sum();
            assert!(metrics > 0.0);

            let error_count = ERROR_COUNT.get();
            assert_eq!(error_count, 0);
        }

        {
            reset_metrics();

            let req = TestRequest::get().uri("/error").to_request();
            let resp = test::call_service(&app, req).await;

            assert_eq!(resp.status(), StatusCode::INTERNAL_SERVER_ERROR);

            let request_count = REQUEST_COUNT
                .with_label_values(&["GET", "/error", "500"])
                .get();
            assert_eq!(request_count, 1);

            let histogram = REQUEST_DURATION.with_label_values(&["GET", "/error", "500"]);
            let metrics = histogram.get_sample_sum();
            assert!(metrics > 0.0);

            let error_count = ERROR_COUNT.get();
            assert_eq!(error_count, 1);
        }
    }
}
