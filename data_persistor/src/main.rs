use async_nats::jetstream::Message;
use common::messaging_config::MessagingConfig;
use common::models::persistence_request::PersistenceRequest;
use common::nats_utils::create_consumer;
use common::setup_logging;
use futures_util::TryStreamExt;
use tracing::{debug, info};

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
    let decoded_payload = PersistenceRequest::from_bytes(&message.message.payload);
    info!("message received: {:?}", decoded_payload);

    // TODO: Save into redis and db
}
