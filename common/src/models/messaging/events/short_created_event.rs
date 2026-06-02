use crate::models::short_url::ShortUrl;
use crate::{ProtoMessage, TypeString};

#[derive(Clone, Debug, ProtoMessage, TypeString)]
#[proto(type = crate::proto::messaging::v1::events::ShortCreatedEvent)]
pub struct ShortCreatedEvent {
    pub short: ShortUrl,
    pub instance_id: String,
}

impl ShortCreatedEvent {
    pub fn new(short: ShortUrl, instance_id: String) -> ShortCreatedEvent {
        ShortCreatedEvent { short, instance_id }
    }
}
