use anyhow::Context;
use testcontainers::{
    core::{IntoContainerPort, WaitFor},
    runners::AsyncRunner,
    GenericImage,
};

const REDIS_IMAGE: (&str, &str) = ("redis", "alpine");
const REDIS_PORT: u16 = 6379;

pub async fn init_redis_container() -> anyhow::Result<String> {
    let redis_container = GenericImage::new(REDIS_IMAGE.0, REDIS_IMAGE.1)
        .with_exposed_port(REDIS_PORT.tcp())
        .with_wait_for(WaitFor::message_on_stdout("Ready to accept connections"))
        .start()
        .await
        .context("Failed to start Redis testcontainer")?;

    let host = redis_container.get_host().await?;
    let url = format!("redis://{host}:{REDIS_PORT}");

    Ok(url)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::caching::{self, CacheValue};

    #[tokio::test]
    async fn test_redis_container_setup() {
        let url = init_redis_container().await.unwrap();
        let redis_client = caching::init_redis(&url).await.unwrap();

        let key = vec![1, 2, 3];
        let value = "Hello World!".to_string();
        caching::cache_entry(&redis_client, key.clone(), CacheValue::Data(value.clone()))
            .await
            .unwrap();

        let cached = caching::get_cached_entry::<String>(&redis_client, &key)
            .await
            .unwrap();
        assert!(cached.is_some());
        assert_eq!(cached.unwrap().value, CacheValue::Data(value));
    }
}
