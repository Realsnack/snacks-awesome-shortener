use r2d2::PooledConnection;
use r2d2_redis::RedisConnectionManager;

use crate::models::short_url::ShortUrl;

pub struct ShortsService {
    redis_connection: PooledConnection<RedisConnectionManager>,
}

impl ShortsService {
    pub fn new(redis_connection: PooledConnection<RedisConnectionManager>) -> ShortsService {
        ShortsService { redis_connection }
    }

    pub fn generate_short(long_url: String) -> ShortUrl {
        let short_url: String = "Habibi".into();

        ShortUrl::new(short_url, long_url, 0)
    }
}
