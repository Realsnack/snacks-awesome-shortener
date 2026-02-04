use crate::models::mongo_short::MongoShortUrl;
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use mongodb::{Client, Collection};
use mongodb::bson::doc;
use tracing::error;

#[async_trait]
pub trait MongoRepository: Send + Sync {
    async fn find_short(&self, key: &str) -> Option<MongoShortUrl>;
    async fn save_short(&self, short_url: MongoShortUrl) -> Result<()>;
    async fn ping_mongo(&self) -> Result<String>;
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
        collection.find_one(doc! { "_id": key}).await.unwrap_or(None)
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

    async fn ping_mongo(&self) -> Result<String> {
        match self.client.database("shorts").run_command(doc! { "ping": 1}).await {
            Ok(response) => Ok(response.to_string()),
            Err(e) => Err(anyhow!("Received mongo error: {}", e))
        }
    }
}
