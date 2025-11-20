use std::sync::Arc;
use crate::services::shorts_service::Shortener;

#[derive(Clone)]
pub struct AppState {
    pub shorts_service: Arc<dyn Shortener>,
}