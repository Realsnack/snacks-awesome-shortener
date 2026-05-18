use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ShortUrl {
    pub short_url: String,
    pub long_url: String,
    pub expiration: i64,
}

impl ShortUrl {
    pub fn new(short_url: String, long_url: String, expiration: i64) -> ShortUrl {
        ShortUrl {
            short_url,
            long_url,
            expiration,
        }
    }

    pub fn to_proto(&self) -> crate::proto::common::v1::ShortUrl {
        crate::proto::common::v1::ShortUrl {
            short_url: self.short_url.clone(),
            long_url: self.long_url.clone(),
            expiration: self.expiration,
        }
    }
}

impl From<crate::proto::common::v1::ShortUrl> for ShortUrl {
    fn from(value: crate::proto::common::v1::ShortUrl) -> Self {
        Self {
            short_url: value.short_url,
            long_url: value.long_url,
            expiration: value.expiration,
        }
    }
}
