use async_nats::jetstream::Message;
use common::config::Config;
use common::models::persistence_request::PersistenceRequest;
use tracing::info;
use common::nats_utils::create_consumer;
use common::setup_logging;

#[tokio::main]
async fn main() -> Result<(), async_nats::Error> {
    setup_logging();
    create_consumer(Config::from_env(), process_message).await?;

    Ok(())
}

pub fn process_message(message: &Message) {
    let decoded_payload = PersistenceRequest::from_vec(&message.message.payload.to_vec());
    info!("message received: {:?}", decoded_payload);
}
