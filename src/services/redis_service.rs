use crate::models::redis_key::RedisKey;

use r2d2::PooledConnection;
use r2d2_redis::{redis::Commands, RedisConnectionManager};
use tide::log;

pub struct RedisService {
    connection_pool: r2d2::Pool<r2d2_redis::RedisConnectionManager>,
}

impl RedisService {
    pub fn new(host: String, port: u16) -> RedisService {
        let connection_manager = r2d2_redis::RedisConnectionManager::new(format!("redis://{}:{}", host, port)).unwrap();

        let connection_pool = r2d2::Pool::builder()
            .max_size(16)
            .build(connection_manager)
            .unwrap();

        RedisService { connection_pool }
    }

    pub fn get_connection(&self) -> PooledConnection<RedisConnectionManager> {
        log::debug!("Getting connection");
        self.connection_pool.get().unwrap()
    }
    
    pub fn get_key(&self, key: &str) -> RedisKey {
        log::debug!("Getting key: {}", key);
        let mut conn = self.connection_pool.get().unwrap();

        // Redis pipeline to get key and get key and expiration in one call
        let (value, expiration) : (Option<String>, u64) = r2d2_redis::redis::pipe()
            .get(key)
            .ttl(key)
            .query(&mut *conn).unwrap_or((None, 0));

        RedisKey::new(key.to_string(), value, Some(expiration))
    }

    pub fn get_key_expiration(&self, key: &str) -> Option<u64> {
        log::debug!("Getting key expiration: {}", key);
        let mut conn = self.connection_pool.get().unwrap();
        let expiration: Option<u64> = conn.ttl(key).unwrap();
        
        expiration
    }

    pub fn set_redis_key(&self, redis_key: RedisKey) {
        log::debug!("Setting key: {} with value: {:?}", redis_key.key, redis_key.value);
        let mut conn = self.connection_pool.get().unwrap();
        let _: () = conn.set(redis_key.key, redis_key.value.unwrap()).unwrap();
    }

    // TODO: Finish counting functions
    pub fn count_redis_keys(&self) -> usize {
        log::debug!("Counting keys");
        let mut conn = self.connection_pool.get().unwrap();
        // let scan_result: usize = conn.scan();
        let scan_result = 0 as usize;

        scan_result
    }

    pub fn count_redis_keys_by_prefix(&self, prefix: String) -> usize {
        log::debug!("Counting keys");
        let mut conn = self.connection_pool.get().unwrap();
        // let scan_result: usize = conn.scan();
        let scan_result = 0 as usize;

        scan_result
    }
}