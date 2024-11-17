use super::counters;
use actix_web::{dev, error};
use std::{pin, task, time};

/// Middleware for collecting Prometheus metrics on HTTP requests.
///
/// Tracks:
/// - Request count (`REQUEST_COUNT`)
/// - Request duration (`REQUEST_DURATION`)
/// - Errors (`ERROR_COUNT`)
///
/// Excludes tracking for specific routes like `/metrics`.
pub struct MetricsMiddleware;

impl<S, B> dev::Transform<S, dev::ServiceRequest> for MetricsMiddleware
where
    S: dev::Service<
        dev::ServiceRequest,
        Response = dev::ServiceResponse<B>,
        Error = actix_web::Error,
    >,
    S::Future: 'static,
{
    type Response = dev::ServiceResponse<B>;
    type Error = actix_web::Error;
    type Transform = MetricsMiddlewareImpl<S>;
    type InitError = ();
    type Future = std::future::Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        std::future::ready(Ok(MetricsMiddlewareImpl { service }))
    }
}

pub struct MetricsMiddlewareImpl<S> {
    service: S,
}

impl<S, B> dev::Service<dev::ServiceRequest> for MetricsMiddlewareImpl<S>
where
    S: dev::Service<
        dev::ServiceRequest,
        Response = dev::ServiceResponse<B>,
        Error = actix_web::Error,
    >,
    S::Future: 'static,
{
    type Response = dev::ServiceResponse<B>;
    type Error = actix_web::Error;
    type Future =
        pin::Pin<Box<dyn std::future::Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&self, ctx: &mut task::Context<'_>) -> task::Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&self, req: dev::ServiceRequest) -> Self::Future {
        let method = req.method().to_string();
        let path = req.path().to_string(); // TODO: Consider dynamic paths (redundant with GraphQL)

        log::debug!("Incoming request: method={}, path={}", method, path);

        if path == "/metrics" {
            return Box::pin(self.service.call(req));
        }

        let start_time = time::Instant::now();
        let fut = self.service.call(req);

        Box::pin(async move {
            let response = fut.await.map_err(|err| {
                counters::ERROR_COUNT.inc();
                log::error!(
                    "Request failed: method={}, path={}, error={:?}",
                    method,
                    path,
                    err
                );
                error::ErrorInternalServerError(err)
            });

            let elapsed = start_time.elapsed().as_secs_f64();

            // Determine HTTP status code
            let status = match &response {
                Ok(res) => res.response().status().as_u16().to_string(),
                Err(_) => {
                    counters::ERROR_COUNT.inc();
                    "500".to_string()
                }
            };

            // Update metrics
            counters::REQUEST_COUNT
                .with_label_values(&[&method, &path, &status])
                .inc();
            counters::REQUEST_DURATION
                .with_label_values(&[&method, &path, &status])
                .observe(elapsed);

            response
        })
    }
}
