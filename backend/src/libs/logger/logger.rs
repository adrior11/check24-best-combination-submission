use actix_web::middleware;
use env_logger;

pub fn init_logging() {
    env_logger::init_from_env(env_logger::Env::new().filter_or("LOG_LEVEL", "info"));
}

pub fn request_logger() -> middleware::Logger {
    middleware::Logger::new("%a \"%r\" %s %b \"%{User-Agent}i\" %T")
}
