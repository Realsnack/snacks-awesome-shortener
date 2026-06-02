use crate::models::short_url::ShortUrl;
use crate::{ProtoMessage, TypeString};

#[derive(Clone, Debug, ProtoMessage, TypeString)]
#[proto(type = crate::proto::messaging::v1::commands::PersistShortCommand)]
pub struct PersistShortCommand {
    pub short: ShortUrl,
    pub created: i64,
}

impl PersistShortCommand {
    pub fn new(short: ShortUrl, created: i64) -> PersistShortCommand {
        PersistShortCommand { short, created }
    }
}
