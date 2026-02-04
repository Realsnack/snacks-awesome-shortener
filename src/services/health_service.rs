use crate::models::health_response::{HealthResponse, HealthStatus, ServiceStatus};
use crate::services::mongo_service::MongoRepository;
use crate::services::redis_service::RedisStore;
use crate::services::{MongoService, RedisService};
use std::collections::HashMap;
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

    pub async fn get_services_health(&self) -> HealthResponse {
        let redis_health = self.get_redis_health().await;
        let mongo_health = self.get_mongo_health().await;

        HealthResponse::new(HashMap::<String, ServiceStatus>::from([
            ("redis".to_string(), redis_health),
            ("mongo".to_string(), mongo_health)
        ]))
    }

    pub async fn get_redis_health(&self) -> ServiceStatus {
        debug!("Getting redis health");
        let (status, message) = match self.redis_service.ping_redis().await {
            Ok(ping_result) => (HealthStatus::HEALTHY, ping_result.to_string()),
            Err(e) => (HealthStatus::UNHEALTHY, e.to_string()),
        };

        let redis_mode = self.redis_service.get_redis_mode();
        let _redis_replication = self.redis_service.get_replication_info().await;

        ServiceStatus::with_details(
            status,
            HashMap::from([
                ("ping".to_string(), message),
                ("redisMode".to_string(), format!("{:?}", redis_mode))
            ])
        )
    }

    pub async fn get_mongo_health(&self) -> ServiceStatus {
        let (status, message) = match self.mongo_service.ping_mongo().await {
            Ok(ping_result) => (HealthStatus::HEALTHY, ping_result.to_string()),
            Err(e) => (HealthStatus::UNHEALTHY, e.to_string()),
        };

        ServiceStatus::with_details(
            status,
            HashMap::from([
                ("ping".to_string(), message)
            ])
        )
    }
}
