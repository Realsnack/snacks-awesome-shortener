use crate::services::ShortsService;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub shorts_service: Arc<ShortsService>,
}