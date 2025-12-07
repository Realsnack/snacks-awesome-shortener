use crate::services::mongo_service::MongoRepository;
use crate::services::redis_service::RedisStore;
use crate::services::{MongoService, RedisService};
use serde_json::{Value, json};
use std::sync::Arc;
use tracing::debug;

pub struct HealthService {
    redis_service: Arc<dyn RedisStore>,
    mongo_service: Arc<dyn MongoRepository>,
}

impl HealthService {
    pub fn new(redis_service: Arc<RedisService>, mongo_service: Arc<MongoService>) -> Self {
        Self {
            redis_service,
            mongo_service,
        }
    }

    pub async fn get_services_health(&self) -> Value {
        let redis_health = self.get_redis_health().await;
        let mongo_health = self.get_mongo_health().await;
        json!({"redis": redis_health, "mongo": mongo_health})
    }

    pub async fn get_redis_health(&self) -> Value {
        debug!("Getting redis health");
        json!({
            "ping": self.redis_service
                .ping_redis()
                .await
                .unwrap_or_else(
                    |e| e.to_string()
                )
        })
    }

    pub async fn get_mongo_health(&self) -> Value {
        debug!("Getting mongo health");
        json!({
            "ping": self.mongo_service
                .ping_mongo()
                .await
                .unwrap_or_else(
                    |e| e.to_string()
                )
        })
    }
}
