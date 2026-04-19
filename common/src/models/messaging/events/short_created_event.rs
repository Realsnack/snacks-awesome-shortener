use serde::{Deserialize, Serialize};
use crate::models::short_url::ShortUrl;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ShortCreatedEvent {
    pub short: ShortUrl,
    pub instance_id: String,
}

impl ShortCreatedEvent {
    pub fn new(short: ShortUrl, instance_id: String) -> ShortCreatedEvent {
        ShortCreatedEvent {
            short,
            instance_id
        }
    }

    pub fn to_vec(&self) -> Result<Vec<u8>, rmp_serde::encode::Error> {
        rmp_serde::to_vec(&self)
    }

    pub fn from_bytes(
        request_bytes: &[u8],
    ) -> Result<ShortCreatedEvent, rmp_serde::decode::Error> {
        rmp_serde::from_slice(request_bytes)
    }
}
