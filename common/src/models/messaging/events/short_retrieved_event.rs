use crate::models::short_url::ShortUrl;
use crate::{ProtoMessage, TypeString};

#[derive(Clone, Debug, ProtoMessage, TypeString)]
#[proto(type = crate::proto::messaging::v1::events::ShortRetrievedEvent)]
pub struct ShortRetrievedEvent {
    pub short: ShortUrl,
    pub instance_id: String,
}

impl ShortRetrievedEvent {
    pub fn new(short: ShortUrl, instance_id: String) -> ShortRetrievedEvent {
        ShortRetrievedEvent { short, instance_id }
    }
}
