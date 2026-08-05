use crate::TypeString;
use crate::models::short_url::ShortUrl;

#[derive(Debug, TypeString)]
pub struct ShortRetrievedEvent {
    pub short: ShortUrl,
    pub instance_id: String,
}

impl ShortRetrievedEvent {
    #[must_use]
    pub const fn new(short: ShortUrl, instance_id: String) -> Self {
        Self { short, instance_id }
    }

    #[must_use]
    pub fn to_proto(&self) -> crate::proto::messaging::v1::events::ShortRetrievedEvent {
        crate::proto::messaging::v1::events::ShortRetrievedEvent {
            short: Some(self.short.to_proto()),
            instance_id: self.instance_id.clone(),
        }
    }
}

impl TryFrom<crate::proto::messaging::v1::events::ShortRetrievedEvent> for ShortRetrievedEvent {
    type Error = crate::proto::proto_conversion_error::ConversionFromNone;

    fn try_from(value: crate::proto::messaging::v1::events::ShortRetrievedEvent) -> Result<Self, crate::proto::proto_conversion_error::ConversionFromNone> {
        let short = value
            .short
            .ok_or(crate::proto::proto_conversion_error::ConversionFromNone)?;

        Ok(Self {
            short: ShortUrl::from(short),
            instance_id: value.instance_id,
        })
    }
}

