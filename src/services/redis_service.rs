use anyhow::Result;
use redis::{AsyncCommands, Client};
use tide::log::error;

#[derive(Clone)]
pub struct RedisService {
    client: Client
}

impl RedisService {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    pub async fn get(&self, key: &str) -> Option<String> {
        match self.client.get_multiplexed_async_connection().await {
            Err(reason) => {
                error!("Error while getting redis connection: {}", reason);
                None
            },
            Ok(mut conn) => {
                match conn.get(key).await {
                    Ok(value) => Some(value),
                    Err(_) => None
                }
            },
        }
    }

    pub async fn set(&self, key: &str, value: &str) -> Result<()> {
        match self.client.get_multiplexed_async_connection().await {
            Err(reason) => {
                error!("Error while getting redis connection: {}", reason);
                Ok(())
            },
            Ok(mut conn) => {
                match conn.set::<_,_,()>(key, value).await {
                    Ok(_) => Ok(()),
                    Err(reason) => {
                        error!("Error while setting redis key: {}", reason);
                        Ok(())
                    }
                }
            }
        }
    }
}