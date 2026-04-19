use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RetrieveShortCommand {
    pub request_time: u64,
    pub short_url: String,
}

impl RetrieveShortCommand {
    pub fn new(request_time: u64, short_url: String) -> RetrieveShortCommand {
        RetrieveShortCommand {
            request_time,
            short_url,
        }
    }

    pub fn to_vec(&self) -> Result<Vec<u8>, rmp_serde::encode::Error> {
        rmp_serde::to_vec(&self)
    }

    pub fn from_bytes(request_bytes: &[u8]) -> Result<RetrieveShortCommand, rmp_serde::decode::Error> {
        rmp_serde::from_slice(request_bytes)
    }
}
