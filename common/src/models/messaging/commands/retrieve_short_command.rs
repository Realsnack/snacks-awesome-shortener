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
