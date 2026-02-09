use crate::models::short_url::ShortUrl;
use mongodb::bson::DateTime;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

#[derive(Debug, Deserialize, Serialize)]
pub struct MongoShortUrl {
    pub _id: String,
    pub long_url: String,
    pub expiration: usize,
    pub created_at: DateTime,
}

impl MongoShortUrl {
    pub fn new(short: ShortUrl) -> Self {
        let now = SystemTime::now();

        MongoShortUrl {
            _id: short.short_url,
            long_url: short.long_url,
            expiration: short.expiration,
            created_at: DateTime::from_system_time(now),
        }
    }
}
