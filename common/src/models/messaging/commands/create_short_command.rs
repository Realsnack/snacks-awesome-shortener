use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Deserialize, Serialize};
use crate::models::rest::CreateShortRequest;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateShortCommand {
    pub request_time: u64,
    pub long_url: String,
    pub expiration: usize,
}

impl CreateShortCommand {
    pub fn new(request_time: u64, long_url: String, expiration: usize) -> CreateShortCommand {
        CreateShortCommand {
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
    ) -> Result<CreateShortCommand, rmp_serde::decode::Error> {
        rmp_serde::from_slice(request_bytes)
    }
}

impl From<CreateShortRequest> for CreateShortCommand {
    fn from(create_short_request: CreateShortRequest) -> Self {
        Self {
            expiration: create_short_request.expiration.unwrap_or(3600),
            long_url: create_short_request.long_url,
            request_time: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
        }
    }
}

impl TryFrom<&[u8]> for CreateShortCommand {
    type Error = rmp_serde::decode::Error;

    fn try_from(request_bytes: &[u8]) -> Result<Self, Self::Error> {
        rmp_serde::from_slice(request_bytes)
    }
}