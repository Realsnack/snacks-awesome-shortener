use snacks_awesome_shortener::services::redis_service::RedisStore;
use std::collections::HashMap;
use std::sync::Arc;
use async_std::sync::Mutex;
use tide::utils::async_trait;

pub struct MockRedis{
    map: Arc<Mutex<HashMap<String,String>>>,
    fail_connect: bool,
    fail_set: bool,
    fail_get: bool,
}

impl MockRedis {
    fn new() -> Self {
        Self {
            map: Arc::new(Mutex::new(HashMap::new())),
            fail_connect: false,
            fail_set: false,
            fail_get: false,
        }
    }

    pub fn with_fail_connect(mut self) -> Self {
        self.fail_connect = true;
        self
    }

    pub fn with_fail_set(mut self) -> Self {
        self.fail_set = true;
        self
    }

    pub fn with_fail_get(mut self) -> Self {
        self.fail_get = true;
        self
    }
}

#[async_trait]
impl RedisStore for MockRedis {
    async fn get(&self, key: &str) -> Option<String> {
        if self.fail_connect || self.fail_get {
            return None;
        }

        let map = self.map.lock().await;
        map.get(key).cloned()
    }

    async fn set(&self, key: &str, value: &str) -> anyhow::Result<()> {
        if self.fail_connect || self.fail_set {
            anyhow::bail!("Mocked redis set failure")
        }

        let mut map = self.map.lock().await;
        map.insert(key.to_string(), value.to_string());
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[async_std::test]
    async fn successful_redis_get() {
        // Arrange
        let redis_mock = MockRedis::new();
        let set_value = "set_value";
        redis_mock.set("test_key", set_value).await.ok();

        // Act
        let redis_result = redis_mock.get("test_key").await.unwrap();

        // Assert
        assert_eq!(set_value, redis_result);
    }

    #[async_std::test]
    async fn successful_redis_set() {
        // Arrange
        let redis_mock = MockRedis::new();
        let set_value = "set_value";

        // Act
        let redis_result = redis_mock.set("test_key", set_value).await.ok();

        // Assert
        assert_ne!(Option::None, redis_result);
    }
}
