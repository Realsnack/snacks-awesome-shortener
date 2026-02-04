use serde::Serialize;
use std::collections::HashMap;

#[derive(Serialize, Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
pub enum HealthStatus {
    HEALTHY = 1,
    DEGRADED = 2,
    UNHEALTHY = 3,
}

#[derive(Serialize)]
pub struct ServiceStatus {
    pub status: HealthStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<HashMap<String, String>>,
}

impl ServiceStatus {
    pub fn new(status: HealthStatus) -> Self {
        ServiceStatus {
            status,
            details: None,
        }
    }

    pub fn with_details(status: HealthStatus, details: HashMap<String, String>) -> Self {
        ServiceStatus {
            status,
            details: Some(details),
        }
    }

    pub fn append_detail(&mut self, key: String, value: String) {
        self.details
            .get_or_insert_with(HashMap::new)
            .insert(key, value);
    }
}

#[derive(Serialize)]
pub struct HealthResponse {
    pub status: HealthStatus,
    pub services: HashMap<String, ServiceStatus>,
}

impl HealthResponse {
    pub fn new(services: HashMap<String, ServiceStatus>) -> Self {
        let mut saw_healthy = false;
        let mut saw_degraded = false;
        let mut saw_unhealthy = false;

        for s in services.values() {
            match s.status {
                HealthStatus::HEALTHY => saw_healthy = true,
                HealthStatus::DEGRADED => saw_degraded = true,
                HealthStatus::UNHEALTHY => saw_unhealthy = true,
            }
        }

        let status = if saw_healthy && !saw_degraded && !saw_unhealthy {
            HealthStatus::HEALTHY
        } else if saw_unhealthy && !saw_healthy && !saw_degraded {
            HealthStatus::UNHEALTHY
        } else {
            HealthStatus::DEGRADED
        };

        HealthResponse { status, services }
    }
}
