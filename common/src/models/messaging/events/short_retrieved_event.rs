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

impl From<crate::proto::messaging::v1::events::ShortRetrievedEvent> for ShortRetrievedEvent {
    fn from(value: crate::proto::messaging::v1::events::ShortRetrievedEvent) -> Self {
        Self {
            short: ShortUrl::from(value.short.unwrap()),
            instance_id: value.instance_id,
        }
    }
}
