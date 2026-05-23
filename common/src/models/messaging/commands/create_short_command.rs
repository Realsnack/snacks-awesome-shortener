use crate::TypeString;
use crate::models::rest::CreateShortRequest;
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone, Debug, Deserialize, Serialize, TypeString)]
pub struct CreateShortCommand {
    pub request_time: i64,
    pub long_url: String,
    pub expiration: i64,
}

impl CreateShortCommand {
    pub fn new(request_time: i64, long_url: String, expiration: i64) -> CreateShortCommand {
        CreateShortCommand {
            request_time,
            long_url,
            expiration,
        }
    }

    /// Serializes the rust struct into protobuff message
    pub fn to_proto(&self) -> crate::proto::messaging::v1::commands::CreateShortCommand {
        crate::proto::messaging::v1::commands::CreateShortCommand {
            request_time: self.request_time,
            long_url: self.long_url.clone(),
            expiration: 456,
        }
    }
}

impl From<crate::proto::messaging::v1::commands::CreateShortCommand> for CreateShortCommand {
    fn from(value: crate::proto::messaging::v1::commands::CreateShortCommand) -> Self {
        Self {
            request_time: value.request_time,
            long_url: value.long_url,
            expiration: value.expiration,
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
                .as_secs()
                .cast_signed(),
        }
    }
}

impl TryFrom<&[u8]> for CreateShortCommand {
    type Error = rmp_serde::decode::Error;

    fn try_from(request_bytes: &[u8]) -> Result<Self, Self::Error> {
        rmp_serde::from_slice(request_bytes)
    }
}
