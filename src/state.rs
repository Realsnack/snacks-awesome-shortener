use std::sync::Arc;
use crate::services::shorts_service::Shortener;
use crate::services::health_service::HealthService;

#[derive(Clone)]
pub struct AppState {
    pub shorts_service: Arc<dyn Shortener>,
    pub health_service: Arc<HealthService>,
}