use std::sync::Arc;
use async_trait::async_trait;
use rand::seq::IteratorRandom;
use rand::rng;
use rand::rngs::ThreadRng;
use tracing::{debug, error};
use crate::models::short_url::ShortUrl;
use crate::services::redis_service::RedisStore;
use crate::services::RedisService;

#[async_trait]
pub trait Shortener: Send + Sync {
    async fn generate_short_url(&self, long_url: String) -> Option<ShortUrl>;
    async fn get_long_url(&self, short_url: String) -> Option<String>;
}

pub struct ShortsService {
    redis_service: Arc<dyn RedisStore>,
}

impl ShortsService {
    pub fn new(redis_service: Arc<RedisService>) -> ShortsService {
        Self { redis_service }
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

        Some(short_url_object)
    }

    async fn get_long_url(&self, short_url: String) -> Option<String> {
        self.redis_service.get(short_url.as_str()).await
    }
}
