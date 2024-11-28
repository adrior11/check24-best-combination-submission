use actix_web::middleware::Logger;
use env_logger::Env;

pub fn init_logging() {
    env_logger::init_from_env(Env::new().filter_or("LOG_LEVEL", "info"));
}

pub fn request_logger() -> Logger {
    Logger::new("%a \"%r\" %s %b \"%{User-Agent}i\" %T")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_request_logger() {
        let logger = request_logger();

        assert!(matches!(logger, Logger { .. }));
    }
}
