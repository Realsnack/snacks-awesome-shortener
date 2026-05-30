use async_nats::HeaderMap;
use async_nats::jetstream::{Context, Message};
use common::config::MessagingConfig;
use common::models::messaging::{CreateShortCommand, PersistShortCommand, ShortCreatedEvent};
use common::models::short_url::ShortUrl;
use common::nats_utils::{create_common_headers, create_consumer, get_header_value};
use common::proto::messaging::v1::commands as protoCommands;
use common::{TypeString, setup_logging};
use futures_util::TryStreamExt;
use prost::Message as _;
use rand::rng;
use rand::rngs::ThreadRng;
use rand::seq::IteratorRandom;
use std::time::SystemTime;
use tracing::{debug, error, info};

#[tokio::main]
async fn main() -> Result<(), async_nats::Error> {
    setup_logging();
    let config = MessagingConfig::from_env(env!("CARGO_PKG_NAME").to_string());
    let client = async_nats::connect(&config.nats_url).await?;
    let jetstream = async_nats::jetstream::new(client);
    let mut consumer_stream = create_consumer(&config, &jetstream).await?;

    while let Ok(Some(message)) = consumer_stream.try_next().await {
        process_message(&message, &config, &jetstream).await;
        message.ack().await?;
    }

    Ok(())
}

pub async fn process_message(message: &Message, config: &MessagingConfig, jetstream: &Context) {
    debug!("Message payload: {:?}", &message);
    let message_type = get_header_value(&message.message.headers, "message_type").unwrap_or("none");

    info!("Received {} message", message_type);

    match message_type {
        "CreateShortCommand" => {
            process_create_short(
                &message.message.payload,
                config,
                message.message.headers.clone().unwrap(),
                jetstream,
            )
            .await
            .unwrap();
        }
        _ => {
            error!("Unsupported message type '{}'", message_type);
        }
    }
}

async fn process_create_short(
    message: &[u8],
    _config: &MessagingConfig,
    header_map: HeaderMap,
    jetstream: &Context,
) -> Result<(), async_nats::Error> {
    let decoded_payload = protoCommands::CreateShortCommand::decode(message)?;
    debug!("decoded payload: {:?}", decoded_payload);
    let converted_payload = CreateShortCommand::from(decoded_payload);
    info!(
        "message received: {:?} with headers {:?}",
        converted_payload, header_map
    );

    let correlation_id = header_map.get("correlation_id").unwrap().to_string();
    let response_subject = header_map.get("response_subject").unwrap().to_string();

    let short = {
        let mut rng = rng();
        ShortUrl::new(
            generate_short(&mut rng),
            converted_payload.long_url.clone(),
            converted_payload.expiration,
        )
    };
    debug!(
        "For url: {} generated short: {:?}",
        converted_payload.long_url, short
    );

    let hostname = std::env::var("HOSTNAME").unwrap_or("unknown".into());
    let created_short_event = ShortCreatedEvent::new(short.clone(), hostname);

    let headers =
        create_common_headers(created_short_event.type_as_string(), correlation_id.clone());

    jetstream
        .publish_with_headers(
            response_subject,
            headers,
            created_short_event.to_proto().encode_to_vec().into(),
        )
        .await?;
    jetstream.client().flush().await?;

    let persistence_request = PersistShortCommand::new(
        short,
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)?
            .as_secs()
            .cast_signed(),
    );

    let headers =
        create_common_headers(persistence_request.type_as_string(), correlation_id.clone());

    jetstream
        .publish_with_headers(
            "data_persistor::request",
            headers,
            persistence_request.to_proto().encode_to_vec().into(),
        )
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
