use std::time::SystemTime;

use crate::TypeString;
use serde::{Deserialize, Serialize};
use tracing::error;

#[derive(Clone, Debug, Deserialize, Serialize, TypeString)]
pub struct RetrieveShortCommand {
    pub request_time: i64,
    pub short_url: String,
}

impl RetrieveShortCommand {
    #[must_use]
    pub fn new(short_url: String) -> Self {
        Self {
            request_time: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH).map_or_else(|err| {
                    error!("System clock is before Unix epoch: {err}");
                    0
                }, |duration| duration.as_secs().cast_signed()),
                short_url,
        }
    }

    #[must_use]
    pub fn to_proto(&self) -> crate::proto::messaging::v1::commands::RetrieveShortCommand {
        crate::proto::messaging::v1::commands::RetrieveShortCommand {
            request_time: self.request_time,
            short_url: self.short_url.clone(),
        }
    }
}

impl From<crate::proto::messaging::v1::commands::RetrieveShortCommand> for RetrieveShortCommand {
    fn from(value: crate::proto::messaging::v1::commands::RetrieveShortCommand) -> Self {
        Self {
            request_time: value.request_time,
            short_url: value.short_url,
        }
    }
}
