use anyhow::{Result, anyhow};
use async_trait::async_trait;
use redis::aio::MultiplexedConnection;
use redis::sentinel::SentinelClient;
use redis::{AsyncCommands, Client, ErrorKind};
use tokio::sync::Mutex;
use tracing::{error, info, warn};

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
pub enum RedisMode {
    Standalone,
    Sentinel,
}

#[async_trait]
pub trait RedisStore: Send + Sync {
    async fn get(&self, key: &str) -> Option<String>;
    async fn set(&self, key: &str, value: &str) -> Result<()>;
    async fn ping_redis(&self) -> Result<String>;
}

pub struct RedisService {
    client: Option<Client>,
    sentinel_client: Option<Mutex<SentinelClient>>,
    redis_mode: RedisMode,
}

impl RedisService {
    pub fn new(
        client: Option<Client>,
        sentinel_client: Option<Mutex<SentinelClient>>,
        redis_mode: RedisMode,
    ) -> Self {
        Self {
            client,
            sentinel_client,
            redis_mode,
        }
    }

    async fn get_connection(&self) -> Option<MultiplexedConnection> {
        match self.redis_mode {
            RedisMode::Standalone => self.client.as_ref()?.get_multiplexed_async_connection().await.ok(),
            RedisMode::Sentinel => {
                let mut sentinel = self.sentinel_client.as_ref()?.lock().await;
                sentinel.get_async_connection().await.ok()
            }
        }
    }
}

#[async_trait]
impl RedisStore for RedisService {
    async fn get(&self, key: &str) -> Option<String> {
        let mut conn = self.get_connection().await?;

        conn.get(key)
            .await
            .map_err(|e| {
                match e.kind() {
                    ErrorKind::TypeError => {
                        info!("Key '{}' was not found", key)
                    }
                    _ => {
                        warn!("Error while getting key '{}' from redis: {}", key, e);
                    }
                };
            })
            .ok()
    }

    async fn set(&self, key: &str, value: &str) -> Result<()> {
        let mut conn = match self.get_connection().await {
            None => return Err(anyhow!("No redis connection")),
            Some(c) => c,
        };

        if let Err(e) = conn.set_ex::<_, _, ()>(key, value, 86400).await {
            error!("Failed to set key '{}': {}", key, e);
            return Err(e.into());
        }

        Ok(())
    }

    async fn ping_redis(&self) -> Result<String> {
        match self.get_connection().await {
            None => return Err(anyhow!("No redis connection")),
            Some(mut c) => Ok(c.ping().await?),
        }
    }
}
