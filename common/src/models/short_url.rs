use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ShortUrl {
    pub short_url: String,
    pub long_url: String,
    pub expiration: u64,
}

impl ShortUrl {
    pub fn new(short_url: String, long_url: String, expiration: u64) -> ShortUrl {
        ShortUrl {
            short_url,
            long_url,
            expiration,
        }
    }

    pub fn to_vec(&self) -> Result<Vec<u8>, rmp_serde::encode::Error> {
        rmp_serde::to_vec(&self)
    }

    pub fn to_proto(&self) -> crate::proto::common::v1::ShortUrl {
        crate::proto::common::v1::ShortUrl {
            short_url: self.short_url.clone(),
            long_url: self.long_url.clone(),
            expiration: self.expiration,
        }
    }
}
