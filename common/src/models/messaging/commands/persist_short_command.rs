use crate::TypeString;
use crate::models::short_url::ShortUrl;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, TypeString)]
pub struct PersistShortCommand {
    pub short: ShortUrl,
    pub created: i64,
}

impl PersistShortCommand {
    #[must_use]
    pub const fn new(short: ShortUrl, created: i64) -> Self {
        Self { short, created }
    }

    #[must_use]
    pub fn to_proto(&self) -> crate::proto::messaging::v1::commands::PersistShortCommand {
        crate::proto::messaging::v1::commands::PersistShortCommand {
            short: Some(self.short.to_proto()),
            created: self.created,
        }
    }
}

impl TryFrom<crate::proto::messaging::v1::commands::PersistShortCommand> for PersistShortCommand {
    type Error = crate::proto::proto_conversion_error::ConversionFromNone;

    fn try_from(value: crate::proto::messaging::v1::commands::PersistShortCommand) -> Result<Self, crate::proto::proto_conversion_error::ConversionFromNone> {
        let short = value
            .short
            .ok_or(crate::proto::proto_conversion_error::ConversionFromNone)?;

        Ok(Self {
            short: ShortUrl::from(short),
            created: value.created
        })
    }
}
