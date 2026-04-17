use dashmap::DashMap;
use std::sync::Arc;
use async_nats::jetstream::Message;
use tokio::sync::oneshot;

#[derive(Clone)]
pub struct AppState {
    pub pending: Arc<DashMap<String, oneshot::Sender<Message>>>,
    pub client: async_nats::Client,
}
