use crate::models::short_url::ShortUrl;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PersistenceRequest {
    short: ShortUrl,
    created: u64,
}

impl PersistenceRequest {
    pub fn new(short: ShortUrl, created: u64) -> PersistenceRequest {
        PersistenceRequest { short, created }
    }

    pub fn to_vec(&self) -> Result<Vec<u8>, rmp_serde::encode::Error> {
        rmp_serde::to_vec(&self)
    }

    pub fn from_vec(
        request_bytes: &Vec<u8>,
    ) -> Result<PersistenceRequest, rmp_serde::decode::Error> {
        rmp_serde::from_slice(request_bytes)
    }
}
