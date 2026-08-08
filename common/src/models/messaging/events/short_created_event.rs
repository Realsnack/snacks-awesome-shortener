use crate::TypeString;
use crate::models::short_url::ShortUrl;

#[derive(Clone, Debug, TypeString)]
pub struct ShortCreatedEvent {
    pub short: ShortUrl,
    pub instance_id: String,
}

impl ShortCreatedEvent {
    #[must_use]
    pub const fn new(short: ShortUrl, instance_id: String) -> Self {
        Self { short, instance_id }
    }

    #[must_use]
    pub fn to_proto(&self) -> crate::proto::messaging::v1::events::ShortCreatedEvent {
        crate::proto::messaging::v1::events::ShortCreatedEvent {
            short: Some(self.short.to_proto()),
            instance_id: self.instance_id.clone(),
        }
    }
}

impl TryFrom<crate::proto::messaging::v1::events::ShortCreatedEvent> for ShortCreatedEvent {
    type Error = crate::proto::proto_conversion_error::ConversionFromNone;

    fn try_from(value: crate::proto::messaging::v1::events::ShortCreatedEvent) -> Result<Self, crate::proto::proto_conversion_error::ConversionFromNone> {
        let short = value
            .short
            .ok_or(crate::proto::proto_conversion_error::ConversionFromNone)?;

        Ok(Self {
            short: ShortUrl::from(short),
            instance_id: value.instance_id,
        })
    }
}

