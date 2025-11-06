use crate::services::{RedisService, ShortsService};
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub shorts_service: Arc<ShortsService>,
    pub redis_service: Arc<RedisService>,
}