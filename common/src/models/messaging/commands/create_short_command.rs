use crate::models::rest::CreateShortRequest;
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateShortCommand {
    pub request_time: u64,
    pub long_url: String,
    pub expiration: u64,
}

impl CreateShortCommand {
    pub fn new(request_time: u64, long_url: String, expiration: u64) -> CreateShortCommand {
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

    /// Serializes the rust struct into protobuff message
    pub fn to_proto(&self) -> crate::proto::messaging::v1::CreateShortCommand {
        crate::proto::messaging::v1::CreateShortCommand {
            request_time: self.request_time,
            long_url: self.long_url.clone(),
            expiration: 456,
        }
    }
}

impl From<CreateShortRequest> for CreateShortCommand {
    fn from(create_short_request: CreateShortRequest) -> Self {
        Self {
            expiration: create_short_request.expiration.unwrap_or(3600),
            long_url: create_short_request.long_url,
            request_time: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }
}

impl TryFrom<&[u8]> for CreateShortCommand {
    type Error = rmp_serde::decode::Error;

    fn try_from(request_bytes: &[u8]) -> Result<Self, Self::Error> {
        rmp_serde::from_slice(request_bytes)
    }
}
