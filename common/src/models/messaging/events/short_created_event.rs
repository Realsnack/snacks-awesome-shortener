use crate::TypeString;
use crate::models::short_url::ShortUrl;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, TypeString)]
pub struct ShortCreatedEvent {
    pub short: ShortUrl,
    pub instance_id: String,
}

impl ShortCreatedEvent {
    pub fn new(short: ShortUrl, instance_id: String) -> ShortCreatedEvent {
        ShortCreatedEvent { short, instance_id }
    }

    pub fn to_proto(&self) -> crate::proto::messaging::v1::events::ShortCreatedEvent {
        crate::proto::messaging::v1::events::ShortCreatedEvent {
            short: Some(self.short.to_proto()),
            instance_id: self.instance_id.clone(),
        }
    }
}

impl From<crate::proto::messaging::v1::events::ShortCreatedEvent> for ShortCreatedEvent {
    fn from(value: crate::proto::messaging::v1::events::ShortCreatedEvent) -> Self {
        Self {
            short: ShortUrl::from(value.short.unwrap()),
            instance_id: value.instance_id,
        }
    }
}
