use dashmap::DashMap;
use std::sync::Arc;
use tokio::sync::oneshot;

#[derive(Clone)]
pub struct AppState {
    pub pending: Arc<DashMap<String, oneshot::Sender<String>>>,
    pub client: async_nats::Client,
}
