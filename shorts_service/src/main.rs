use async_nats::jetstream::Message;
use common::config::Config;
use common::models::create_short_request::CreateShortRequest;
use common::models::get_short_request::GetShortRequest;
use common::models::persistence_request::PersistenceRequest;
use common::models::short_url::ShortUrl;
use common::nats_utils::create_consumer;
use common::setup_logging;
use futures_util::TryStreamExt;
use rand::rngs::ThreadRng;
use rand::seq::IteratorRandom;
use rand::thread_rng;
use std::time::SystemTime;
use tracing::{debug, error, info};

#[tokio::main]
async fn main() -> Result<(), async_nats::Error> {
    setup_logging();
    let config = Config::from_env(env!("CARGO_PKG_NAME").to_string());
    let mut consume_stream = create_consumer(&config).await?;

    while let Ok(Some(message)) = consume_stream.try_next().await {
        process_message(&message, &config).await;
        message.ack().await?;
    }

    Ok(())
}

pub async fn process_message(message: &Message, config: &Config) {
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
            process_create_short(&message.message.payload, config)
                .await
                .unwrap();
        }
        _ => {
            error!("Unsupported message type '{}'", message_type);
        }
    }
}

fn process_get_short(message: &bytes::Bytes) {
    let decoded_payload = GetShortRequest::from_vec(&message.to_vec()).unwrap();
    info!("message received: {:?}", decoded_payload);
}

async fn process_create_short(
    message: &bytes::Bytes,
    config: &Config,
) -> Result<(), async_nats::Error> {
    let decoded_payload = CreateShortRequest::from_vec(&message.to_vec())?;
    info!("message received: {:?}", decoded_payload);

    let short = {
        let mut rng = thread_rng();
        ShortUrl::new(
            generate_short(&mut rng),
            decoded_payload.long_url.clone(),
            3600,
        )
    };
    debug!(
        "For url: {} generated short: {:?}",
        decoded_payload.long_url, short
    );

    let client = async_nats::connect(&config.nats_url).await?;
    let jetstream = async_nats::jetstream::new(client);

    let persistence_request = PersistenceRequest::new(
        short,
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)?
            .as_secs(),
    )
    .to_vec()?;

    jetstream
        .publish("data_persistor::request", persistence_request.into())
        .await?;
    jetstream.client().flush().await?;

    Ok(())
}

pub async fn publish_jetstream_message(
    config: Config,
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
