use crate::services::mongo_service::MongoRepository;
use crate::services::redis_service::RedisStore;
use crate::services::{MongoService, RedisService};
use serde_json::{Value, json};
use std::sync::Arc;
use tracing::debug;

pub struct HealthService {
    redis_service: Arc<dyn RedisStore>,
    _mongo_service: Arc<dyn MongoRepository>,
}

impl HealthService {
    pub fn new(redis_service: Arc<RedisService>, mongo_service: Arc<MongoService>) -> Self {
        Self {
            redis_service,
            _mongo_service: mongo_service,
        }
    }

    pub async fn get_services_health(&self) -> Value {
        let redis_health = self.get_redis_health().await;
        let mongo_health = self.get_mongo_health().await;
        json!({"redis": redis_health, "mongo": mongo_health})
    }

    pub async fn get_redis_health(&self) -> Value {
        debug!("Getting redis health");
        match self.redis_service.ping_redis().await {
            Ok(ping_result) => {
                json!({"PING": ping_result})
            }
            Err(_) => {
                json!({"PING": "error"})
            }
        }
    }

    pub async fn get_mongo_health(&self) -> Value{
        json!({"health": "unknown"})
    }
}
