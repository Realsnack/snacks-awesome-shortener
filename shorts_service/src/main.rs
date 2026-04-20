use async_nats::HeaderMap;
use async_nats::jetstream::Message;
use common::messaging_config::MessagingConfig;
use common::models::messaging::{
    CreateShortCommand, PersistShortCommand, RetrieveShortCommand, ShortCreatedEvent,
};
use common::models::short_url::ShortUrl;
use common::nats_utils::create_consumer;
use common::setup_logging;
use futures_util::TryStreamExt;
use rand::rng;
use rand::rngs::ThreadRng;
use rand::seq::IteratorRandom;
use std::time::SystemTime;
use tracing::{debug, error, info};

#[tokio::main]
async fn main() -> Result<(), async_nats::Error> {
    setup_logging();
    let config = MessagingConfig::from_env(env!("CARGO_PKG_NAME").to_string());
    let mut consumer_stream = create_consumer(&config).await?;

    while let Ok(Some(message)) = consumer_stream.try_next().await {
        process_message(&message, &config).await;
        message.ack().await?;
    }

    Ok(())
}

pub async fn process_message(message: &Message, config: &MessagingConfig) {
    debug!("Message payload: {:?}", &message);
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
        "GetShortRequest" => {
            process_get_short(&message.message.payload);
        }
        "CreateShortRequest" => {
            process_create_short(
                &message.message.payload,
                config,
                message.message.headers.clone().unwrap(),
            )
            .await
            .unwrap();
        }
        _ => {
            error!("Unsupported message type '{}'", message_type);
        }
    }
}

fn process_get_short(message: &bytes::Bytes) {
    let decoded_payload = RetrieveShortCommand::from_bytes(message).unwrap();
    info!("message received: {:?}", decoded_payload);
}

async fn process_create_short(
    message: &bytes::Bytes,
    config: &MessagingConfig,
    header_map: HeaderMap,
) -> Result<(), async_nats::Error> {
    let decoded_payload = CreateShortCommand::from_bytes(message)?;
    info!(
        "message received: {:?} with headers {:?}",
        decoded_payload, header_map
    );

    let correlation_id = header_map.get("correlation_id").unwrap().to_string();
    let response_subject = header_map.get("response_subject").unwrap().to_string();

    let short = {
        let mut rng = rng();
        ShortUrl::new(
            generate_short(&mut rng),
            decoded_payload.long_url.clone(),
            decoded_payload.expiration,
        )
    };
    debug!(
        "For url: {} generated short: {:?}",
        decoded_payload.long_url, short
    );

    let client = async_nats::connect(&config.nats_url).await?;
    let jetstream = async_nats::jetstream::new(client);

    let hostname = std::env::var("HOSTNAME").unwrap_or("unknown".into());
    let created_short_event = ShortCreatedEvent::new(short.clone(), hostname).to_vec()?;

    let mut headers = HeaderMap::new();
    headers.insert("message_type", "CreatedShortResponse");
    headers.insert("correlation_id", correlation_id.to_string());

    jetstream
        .publish_with_headers(
            response_subject,
            headers.clone(),
            created_short_event.into(),
        )
        .await?;
    jetstream.client().flush().await?;

    let persistence_request = PersistShortCommand::new(
        short,
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)?
            .as_secs(),
    )
    .to_vec()?;

    jetstream
        .publish_with_headers(
            "data_persistor::request",
            headers,
            persistence_request.into(),
        )
        .await?;
    jetstream.client().flush().await?;

    Ok(())
}

pub async fn publish_jetstream_message(
    config: MessagingConfig,
    message: Vec<u8>,
) -> Result<(), async_nats::Error> {
    let client = async_nats::connect(config.nats_url).await?;
    let jetstream = async_nats::jetstream::new(client);

    jetstream
        .publish("data_persistor::request", message.into())
        .await?;
    jetstream.client().flush().await?;

    Ok(())
}

fn generate_short(mut rng: &mut ThreadRng) -> String {
    const CHARS: &str = "abcdefghjklmnopqrtuvwxyzABCDEFGHJKLMNOPQRTUVWXYZ1234567890";
    let short_url: String = (0..6)
        .map(|_| CHARS.chars().choose(&mut rng).unwrap())
        .collect();

    short_url
}
