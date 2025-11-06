use anyhow::Result;
use redis::{AsyncCommands, Client};

#[derive(Clone)]
pub struct RedisService {
    client: Client
}

impl RedisService {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    pub async fn get(&self, key: &str) -> Result<Option<String>> {
        let mut conn = self.client.get_multiplexed_async_connection().await?;
        Ok(conn.get(key).await?)
    }

    pub async fn set(&self, key: &str, value: &str) -> Result<()> {
        let mut conn = self.client.get_multiplexed_async_connection().await?;
        conn.set::<_,_,()>(key, value).await?;
        Ok(())
    }
}