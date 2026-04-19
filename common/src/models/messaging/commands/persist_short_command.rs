use crate::models::short_url::ShortUrl;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PersistShortCommand {
    short: ShortUrl,
    created: u64,
}

impl PersistShortCommand {
    pub fn new(short: ShortUrl, created: u64) -> PersistShortCommand {
        PersistShortCommand { short, created }
    }

    pub fn to_vec(&self) -> Result<Vec<u8>, rmp_serde::encode::Error> {
        rmp_serde::to_vec(&self)
    }

    pub fn from_bytes(
        request_bytes: &[u8],
    ) -> Result<PersistShortCommand, rmp_serde::decode::Error> {
        rmp_serde::from_slice(request_bytes)
    }
}
