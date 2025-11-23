use anyhow::Result;
use async_trait::async_trait;
use redis::{AsyncCommands, Client};
use redis::aio::MultiplexedConnection;
use tracing::error;

#[async_trait]
pub trait RedisStore: Send + Sync {
    async fn get(&self, key: &str) -> Option<String>;
    async fn set(&self, key: &str, value: &str) -> Result<()>;
}

#[derive(Clone)]
pub struct RedisService {
    client: Client
}

impl RedisService {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    async fn get_connection(&self) -> Option<MultiplexedConnection> {
        self.client
            .get_multiplexed_async_connection()
            .await
            .map_err(|e| {
                error!("Error while getting redis connection: {}", e);
            })
            .ok()
    }
}

#[async_trait]
impl RedisStore for RedisService {
    async fn get(&self, key: &str) -> Option<String> {
        let mut conn = self.get_connection().await?;

        conn.get(key)
        .await
        .map_err(|e| {
            error!("Error while getting key '{}' from redis: {}", key, e);
        })
        .ok()
    }

    async fn set(&self, key: &str, value: &str) -> Result<()> {
        let mut conn = match self.get_connection().await {
            None => return Err(anyhow::anyhow!("No redis connection")),
            Some(c) => c
        };

        if let Err(e) =conn.set_ex::<_,_,()>(key,value, 86400).await {
            error!("Failed to set key '{}': {}", key, e);
            return Err(e.into());
        }

        Ok(())
    }
}