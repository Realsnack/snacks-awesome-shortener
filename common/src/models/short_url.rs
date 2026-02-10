use crate::models::mongo_short::MongoShortUrl;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ShortUrl {
    pub short_url: String,
    pub long_url: String,
    pub expiration: usize,
}

impl ShortUrl {
    pub fn new(short_url: String, long_url: String, expiration: usize) -> ShortUrl {
        ShortUrl {
            short_url,
            long_url,
            expiration,
        }
    }

    pub fn from_mongo_short(mongo_short: MongoShortUrl) -> ShortUrl {
        ShortUrl {
            short_url: mongo_short._id,
            long_url: mongo_short.long_url,
            expiration: mongo_short.expiration,
        }
    }

    pub fn to_vec(&self) -> Result<Vec<u8>, rmp_serde::encode::Error> {
        rmp_serde::to_vec(&self)
    }
}
