use crate::TypeString;
use crate::models::short_url::ShortUrl;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, TypeString)]
pub struct PersistShortCommand {
    pub short: ShortUrl,
    pub created: i64,
}

impl PersistShortCommand {
    pub fn new(short: ShortUrl, created: i64) -> PersistShortCommand {
        PersistShortCommand { short, created }
    }

    pub fn to_proto(&self) -> crate::proto::messaging::v1::commands::PersistShortCommand {
        crate::proto::messaging::v1::commands::PersistShortCommand {
            short: Some(self.short.to_proto()),
            created: self.created,
        }
    }
}

impl From<crate::proto::messaging::v1::commands::PersistShortCommand> for PersistShortCommand {
    fn from(value: crate::proto::messaging::v1::commands::PersistShortCommand) -> Self {
        Self {
            short: ShortUrl::from(value.short.unwrap()),
            created: value.created,
        }
    }
}
