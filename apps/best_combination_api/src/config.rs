use once_cell::sync::Lazy;
use serde::Deserialize;

#[derive(Clone, Deserialize, Debug)]
pub struct Config {
    pub mongodb_uri: String,
    pub redis_url: String,
    pub rabbitmq_url: String,
    pub task_queue_name: String,
}

pub static CONFIG: Lazy<Config> = Lazy::new(|| {
    envy::from_env::<Config>()
        .unwrap_or_else(|err| panic!("Failed to load configuration from env: {:#?}", err))
});

#[cfg(test)]
mod tests {
    use super::*;
    use std::panic;

    #[test]
    fn test_config_initialization() {
        dotenv::dotenv().ok();

        // Catch any panic to ensure initialization does not fail
        let result = panic::catch_unwind(|| {
            &*CONFIG // Force evaluation of CONFIG
        });

        assert!(result.is_ok(), "CONFIG failed to initialize");
        let cfg = result.unwrap();

        assert!(
            !format!("{:?}", cfg).is_empty(),
            "CONFIG is empty or invalid"
        );
    }
}
