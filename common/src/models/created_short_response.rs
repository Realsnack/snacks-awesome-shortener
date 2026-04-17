use serde::{Deserialize, Serialize};
use crate::models::persistence_request::PersistenceRequest;
use crate::models::short_url::ShortUrl;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreatedShortResponse {
    pub short: ShortUrl,
    pub instance_id: String,
}

impl CreatedShortResponse {
    pub fn new(short: ShortUrl, instance_id: String) -> CreatedShortResponse {
        CreatedShortResponse {
            short,
            instance_id
        }
    }

    pub fn to_vec(&self) -> Result<Vec<u8>, rmp_serde::encode::Error> {
        rmp_serde::to_vec(&self)
    }

    pub fn from_bytes(
        request_bytes: &[u8],
    ) -> Result<CreatedShortResponse, rmp_serde::decode::Error> {
        rmp_serde::from_slice(request_bytes)
    }
}
