use async_trait::async_trait;
use mongodb::Client;
use crate::models::short_url::ShortUrl;

#[async_trait]
pub trait MongoRepository: Send + Sync {
    async fn find_short(&self, key: &str) -> Option<ShortUrl>;
    async fn save_short(&self, short_url: ShortUrl) -> anyhow::Result<()>;
}

#[derive(Clone)]
pub struct MongoService {
    client: Client,
}

impl MongoService{
    pub fn new(client: Client) -> Self {
        Self { client }
    }
}
