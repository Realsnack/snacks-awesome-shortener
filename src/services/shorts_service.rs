use std::sync::Arc;
use rand::seq::IteratorRandom;
use rand::rng;
use rand::rngs::ThreadRng;
use tide::log::debug;
use crate::models::short_url::ShortUrl;
use crate::services::RedisService;
use serde_json::json;

pub struct ShortsService {
    redis_service: Arc<RedisService>,
}

impl ShortsService {
    pub fn new(redis_service: Arc<RedisService>) -> ShortsService {
        Self { redis_service }
    }

    pub async fn generate_short_url(&self, long_url: String) -> ShortUrl {
        const CHARS: &str = "abcdefghjklmnopqrtuvwxyzABCDEFGHJKLMNOPQRTUVWXYZ1234567890";
        let short_url = {
            let mut rng = rng();
            Self::generate_short(&mut rng)
        };

        debug!("For url: {} generated short: {}", long_url, short_url);

        let short_url_object = ShortUrl::new(short_url.clone(), long_url, 86400);

        self.redis_service.set(short_url.clone().as_str(), serde_json::to_string(&short_url_object).unwrap_or_default().as_str()).await.unwrap_or_default();

        short_url_object
    }

    fn generate_short(mut rng: &mut ThreadRng) -> String {
        const CHARS: &str = "abcdefghjklmnopqrtuvwxyzABCDEFGHJKLMNOPQRTUVWXYZ1234567890";
        let short_url: String = (0..6)
            .map(|_| CHARS.chars().choose(&mut rng).unwrap())
            .collect();

        short_url
    }
    pub async fn get_long_url(&self, short_url: String) -> String {
        let short_url_string = self.redis_service.get(short_url.as_str()).await.unwrap_or_default().unwrap_or_default();

        short_url_string
    }
}
