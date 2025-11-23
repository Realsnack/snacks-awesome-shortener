use crate::models::mongo_short::MongoShortUrl;
use anyhow::Result;
use async_trait::async_trait;
use mongodb::{Client, Collection};
use mongodb::bson::doc;
use tracing::error;

#[async_trait]
pub trait MongoRepository: Send + Sync {
    async fn find_short(&self, key: &str) -> Option<MongoShortUrl>;
    async fn save_short(&self, short_url: MongoShortUrl) -> anyhow::Result<()>;
}

pub struct MongoService {
    client: Client,
}

impl MongoService {
    pub fn new(client: Client) -> Self {
        Self { client }
    }
}

#[async_trait]
impl MongoRepository for MongoService {
    async fn find_short(&self, key: &str) -> Option<MongoShortUrl> {
        let collection: Collection<MongoShortUrl> =
            self.client.database("shorts").collection("short_url");
        collection.find_one(doc! { "_id": key}).await.unwrap_or_else(|_| None)
    }

    async fn save_short(&self, short: MongoShortUrl) -> Result<()> {
        let collection: Collection<MongoShortUrl> =
            self.client.database("shorts").collection("short_url");
        if let Err(e) = collection.insert_one(short).await {
            error!("Failed to save short to mongo: {}", e);
            return Err(e.into());
        }

        Ok(())
    }
}
