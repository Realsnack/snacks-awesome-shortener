use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateShortRequest {
    pub request_time: u64,
    pub long_url: String,
    pub expiration: usize,
}

impl CreateShortRequest {
    pub fn new(request_time: u64, long_url: String, expiration: usize) -> CreateShortRequest {
        CreateShortRequest {
            request_time,
            long_url,
            expiration
        }
    }

    pub fn to_vec(&self) -> Result<Vec<u8>, rmp_serde::encode::Error> {
        rmp_serde::to_vec(&self)
    }

    pub fn from_vec(request_bytes: &Vec<u8>) -> Result<CreateShortRequest, rmp_serde::decode::Error> {
        rmp_serde::from_slice(request_bytes)
    }
}