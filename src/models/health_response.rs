use std::collections::HashMap;
use serde::Serialize;

#[derive(Serialize)]
pub struct HealthResponse {
    pub status: HealthStatus,
    pub services: HashMap<String, ServiceStatus>,
}

#[derive(Serialize)]
pub struct ServiceStatus {
    pub status: HealthStatus,
    pub details: HashMap<String, String>
}

#[derive(Serialize)]
pub enum HealthStatus {
    UP,
    DEGRADED,
    DOWN
}
