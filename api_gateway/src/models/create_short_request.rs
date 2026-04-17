use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateShortRequest {
    pub long_url: String,
    pub expiration: Option<usize>,
}

impl CreateShortRequest {
    pub fn new(request_time: u64, long_url: String, expiration: usize) -> CreateShortRequest {
        CreateShortRequest {
            request_time,
            long_url,
            expiration,
        }
    }

    pub fn to_vec(&self) -> Result<Vec<u8>, rmp_serde::encode::Error> {
        rmp_serde::to_vec(&self)
    }

    pub fn from_bytes(
        request_bytes: &[u8],
    ) -> Result<CreateShortRequest, rmp_serde::decode::Error> {
        rmp_serde::from_slice(request_bytes)
    }
}
