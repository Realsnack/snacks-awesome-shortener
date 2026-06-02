use std::time::SystemTime;

use crate::{ProtoMessage, TypeString};

#[derive(Clone, Debug, ProtoMessage, TypeString)]
#[proto(type = crate::proto::messaging::v1::commands::RetrieveShortCommand)]
pub struct RetrieveShortCommand {
    pub request_time: i64,
    pub short_url: String,
}

impl RetrieveShortCommand {
    pub fn new(short_url: String) -> RetrieveShortCommand {
        RetrieveShortCommand {
            request_time: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs()
                .cast_signed(),
            short_url,
        }
    }
}
