use actix_web::middleware::Logger;
use env_logger::Env;

/// Initializes the global logger.
///
/// This function configures the logger using environment variables.
///
/// # Panics
///
/// This function will panic if it is called more than once, or if another
/// library has already initialized a global logger. Ensure this function
/// is only called once during applications lifetime.
pub fn init_logging() {
    env_logger::init_from_env(Env::new().filter_or("LOG_LEVEL", "info"));
}

/// Configures the Actix Web request logger.
///
/// Returns a Logger middleware instance that logs request and response details.
pub fn request_logger() -> Logger {
    Logger::new("%a \"%r\" %s %b \"%{User-Agent}i\" %T")
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test as actix_test, web, App, HttpResponse};

    #[test]
    fn test_request_logger() {
        let logger = request_logger();

        assert!(matches!(logger, Logger { .. }));
    }

    #[actix_web::test]
    async fn test_service_logging() {
        dotenv::dotenv().ok();

        init_logging();

        async fn test_handler() -> HttpResponse {
            HttpResponse::Ok().body("Hello World!")
        }

        let service = actix_test::init_service(
            App::new()
                .wrap(request_logger())
                .route("/test", web::get().to(test_handler)),
        )
        .await;

        let request = actix_test::TestRequest::get().uri("/test").to_request();
        let response = actix_test::call_service(&service, request).await;

        assert!(response.status().is_success());
        assert_eq!(response.status(), 200);
    }
}
