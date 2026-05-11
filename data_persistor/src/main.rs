use std::str::Bytes;

use async_nats::jetstream::Message;
use common::messaging_config::MessagingConfig;
use common::models::messaging::PersistShortCommand;
use common::nats_utils::create_consumer;
use common::setup_logging;
use futures_util::TryStreamExt;
use tracing::{debug, error, info};

#[tokio::main]
async fn main() -> Result<(), async_nats::Error> {
    setup_logging();
    let config = MessagingConfig::from_env(env!("CARGO_PKG_NAME").to_string());
    let mut consumer_stream = create_consumer(&config).await?;

    while let Ok(Some(message)) = consumer_stream.try_next().await {
        process_message(&message).await;
        message.ack().await?;
    }

    Ok(())
}

pub async fn process_message(message: &Message) {
    debug!("Message payload: {:?}", &message.message);

    let message_type = match &message.headers {
        None => {
            error!("No headers in message: {:?}", &message);
            "none"
        }
        Some(headers) => match headers.get("message_type") {
            None => {
                error!(
                    "No 'message_type' header in message: {:?}",
                    &message.message
                );
                "none"
            }
            Some(message_type) => message_type.as_str(),
        },
    };

    info!("Received {} message", message_type);

    match message_type {
        "PersistShortCommand" => {}
        _ => {
            error!("Unsupported message type '{}'", message_type);
        }
    }
}

pub async fn persist_short_command(message: &bytes::Bytes) {}
