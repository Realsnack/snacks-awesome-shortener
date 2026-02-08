use async_nats::jetstream::Message;
use common::config::Config;
use tracing::{debug, error, info};
use common::models::create_short_request::CreateShortRequest;
use common::models::persistence_request::PersistenceRequest;
use common::nats_utils::create_consumer;
use common::setup_logging;

#[tokio::main]
async fn main() -> Result<(), async_nats::Error> {
    setup_logging();
    create_consumer(Config::from_env(), process_message).await?;

    Ok(())
}

pub fn process_message(message: &Message) {
    debug!("Message payload: {:?}", &message);
    let message_type = match &message.headers {
        None => {
            error!("No headers in message: {:?}", &message);
            "none"
        }
        Some(headers) => {
            match headers.get("message_type") {
                None => {
                    error!("No 'message_type' header in message: {:?}", &message.message);
                    "none"
                }
                Some(message_type) => {
                    message_type.as_str()
                }
            }
        }
    };

    info!("Received {} message", message_type);

    match message_type {
        "GetShortRequest" => {
            process_get_short(&message.message.payload);
        },
        "CreateShortRequest" => {
            process_create_short(&message.message.payload);
        },
        _ => {
            error!("Unsupported message type '{}'", message_type);
        }
    }
}

pub fn process_get_short(message: &bytes::Bytes) {
    todo!();
}

pub fn process_create_short(message: &bytes::Bytes) {
    let decoded_payload = CreateShortRequest::from_vec(&message.to_vec());
    info!("message received: {:?}", decoded_payload);
}
