use anyhow::Result;
use std::sync::Arc;
use async_trait::async_trait;
use rand::seq::IteratorRandom;
use rand::rng;
use rand::rngs::ThreadRng;
use serde_json::json;
use tracing::{debug, error, info};
use crate::models::mongo_short::MongoShortUrl;
use crate::models::short_url::ShortUrl;
use crate::services::redis_service::RedisStore;
use crate::services::{MongoService, RedisService};
use crate::services::mongo_service::MongoRepository;

#[async_trait]
pub trait Shortener: Send + Sync {
    async fn generate_short_url(&self, long_url: String) -> Option<ShortUrl>;
    async fn get_long_url(&self, short_url: String) -> Option<String>;
    async fn save_short_to_mongo(&self, short: ShortUrl) -> Result<()>;
    async fn get_short_from_mongo(&self, short_url: &str) -> Option<ShortUrl>;
}

pub struct ShortsService {
    redis_service: Arc<dyn RedisStore>,
    mongo_service: Arc<dyn MongoRepository>,
}

impl ShortsService {
    pub fn new(redis_service: Arc<RedisService>, mongo_service: Arc<MongoService>) -> ShortsService {
        Self { redis_service, mongo_service }
    }

    fn generate_short(mut rng: &mut ThreadRng) -> String {
        const CHARS: &str = "abcdefghjklmnopqrtuvwxyzABCDEFGHJKLMNOPQRTUVWXYZ1234567890";
        let short_url: String = (0..6)
            .map(|_| CHARS.chars().choose(&mut rng).unwrap())
            .collect();

        short_url
    }
}

#[async_trait]
impl Shortener for ShortsService {
    async fn generate_short_url(&self, long_url: String) -> Option<ShortUrl> {
        let short_url = {
            let mut rng = rng();
            Self::generate_short(&mut rng)
        };

        debug!("For url: {} generated short: {}", long_url, short_url);

        let short_url_object = ShortUrl::new(short_url.clone(), long_url, 86400);

        let payload = match serde_json::to_string(&short_url_object) {
            Ok(val) => val,
            Err(e) => {
                error!("Failed to serialize ShortUrl: {}", e);
                return None;
            }
        };

        if let Err(e) = self.redis_service.set(&short_url, &payload).await {
            error!("Failed to save short url {}: {}", short_url, e);
            return None;
        }

        if let Err(e) = self.save_short_to_mongo(short_url_object.clone()).await {
            error!("Failed to save mongo object: {}", e);
        };

        Some(short_url_object)
    }

    async fn get_long_url(&self, short_url: String) -> Option<String> {
        if let Some(long_url) = self.redis_service.get(short_url.as_str()).await {
            return Some(long_url)
        }

        info!("Short '{}' not found in redis, trying mongo", short_url);
        match self.get_short_from_mongo(&short_url).await.map(|s| json!(s).to_string()) {
            None => None,
            Some(short_url_string) => {
                info!("Short found in mongo, saving to redis");
                if let Err(e) = self.redis_service.set(short_url.as_str(), short_url_string.as_str()).await {
                    error!("Failed to save short to redis: {}", e);
                }
                Some(short_url_string)
            }
        }
    }

    async fn save_short_to_mongo(&self, short: ShortUrl) -> Result<()> {
        let mongo_short = MongoShortUrl::new(short);

        Ok(self.mongo_service.save_short(mongo_short).await?)
    }

    async fn get_short_from_mongo(&self, short_url: &str) -> Option<ShortUrl> {
        self.mongo_service.find_short(short_url).await.map(ShortUrl::from_mongo_short)
    }
}
