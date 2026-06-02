use std::time::{SystemTime, UNIX_EPOCH};

use crate::models::rest::CreateShortRequest;
use crate::{ProtoMessage, TypeString};

#[derive(Clone, Debug, ProtoMessage, TypeString)]
#[proto(type = crate::proto::messaging::v1::commands::CreateShortCommand)]
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
