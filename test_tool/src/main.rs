use async_nats::HeaderMap;
use async_nats::jetstream::Context;
use clap::{Parser, Subcommand};
use common::messaging_config::MessagingConfig;
use common::models::messaging::{
    CreateShortCommand, PersistShortCommand, RetrieveShortCommand, ShortCreatedEvent,
};
use common::models::short_url::ShortUrl;
use common::nats_utils::create_consumer;
use common::{TypeString, setup_logging};
use futures_util::TryStreamExt;
use prost::Message;
use std::time::SystemTime;
use tracing::{debug, info};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    pub command: Commands,
}

// FIXME: Update enum for proto Commands
#[derive(Debug, Subcommand)]
enum Commands {
    SendCreateShortRequest,
    SendPersistenceRequest,
    SendShortCreatedResponse,
    SendRetrieveShortCommand,
    ConsumeShortCreatedResponse,
}

async fn setup_jetstream(nats_url: &str) -> Result<Context, async_nats::Error> {
    info!("Connecting to: '{}'", nats_url);
    let client = async_nats::connect(nats_url).await?;
    Ok(async_nats::jetstream::new(client))
}

async fn send_persistence_request(jetstream: Context) -> Result<(), async_nats::Error> {
    let short_url = ShortUrl::new("asdfgkh".to_string(), "https://hltv.org".to_string(), 600);
    let data = PersistShortCommand::new(
        short_url,
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)?
            .as_secs()
            .cast_signed(),
    );
    let mut headers = HeaderMap::new();
    headers.insert("message_type", data.type_as_string());
    headers.insert("correlation_id", "test-tool");

    info!("Publishing message: {:?}", data);
    jetstream
        .publish_with_headers(
            "data_persistor::request",
            headers,
            data.to_proto().encode_to_vec().into(),
        )
        .await?;
    jetstream.client().flush().await?;

    Ok(())
}

async fn send_retrieve_short_command(jetstream: Context) -> Result<(), async_nats::Error> {
    let mut headers = HeaderMap::new();

    let data = RetrieveShortCommand::new(
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)?
            .as_secs()
            .cast_signed(),
        String::from("1234"),
    );

    headers.insert("message_type", data.type_as_string());
    headers.insert("correlation_id", "test-tool");
    debug!("Headers set: {:?}", headers);

    info!("Publishing message: {:?}", data);
    jetstream
        .publish_with_headers(
            "data_persistor::request",
            headers,
            data.to_proto().encode_to_vec().into(),
        )
        .await?;
    jetstream.client().flush().await?;

    Ok(())
}

async fn send_create_short_request(jetstream: Context) -> Result<(), async_nats::Error> {
    let create_short_request = CreateShortCommand::new(
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)?
            .as_secs()
            .cast_signed(),
        "https://hltv.org/".into(),
        3600,
    );
    let mut headers = HeaderMap::new();
    headers.insert("message_type", create_short_request.type_as_string());

    info!("Publishing message: {:?}", create_short_request);
    jetstream
        .publish_with_headers(
            "shorts_service::request",
            headers,
            create_short_request.to_proto().encode_to_vec().into(),
        )
        .await?;
    jetstream.client().flush().await?;

    Ok(())
}

async fn send_short_created_response(jetstream: Context) -> Result<(), async_nats::Error> {
    let short = ShortUrl::new("/retcd".into(), "http://hltv.org/".into(), 86400);
    let created_short = ShortCreatedEvent::new(short, "test_tool".into());
    let mut headers = HeaderMap::new();
    headers.insert("message_type", created_short.type_as_string());
    headers.insert("correlation_id", "test-tool");

    info!("Publishing message: {:?}", created_short);
    jetstream
        .publish_with_headers(
            "api_gateway::response",
            headers,
            created_short.to_proto().encode_to_vec().into(),
        )
        .await?;
    jetstream.client().flush().await?;

    Ok(())
}

async fn consume_short_created_response(nats_url: String) -> Result<(), async_nats::Error> {
    let config = MessagingConfig::new(
        "".to_string(),
        "short_service::response".to_string(),
        "test-tool".to_string(),
        nats_url,
        10000,
    );
    let mut consumer_stream = create_consumer(&config).await?;

    while let Ok(Some(message)) = consumer_stream.try_next().await {
        info!("Received message {:?}", message);
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), async_nats::Error> {
    setup_logging();
    let args = Args::parse();
    let nats_url = "localhost:4222";
    let jetstream = setup_jetstream(nats_url).await?;

    info!("Action chosen {:?}", args.command);

    match args.command {
        Commands::SendPersistenceRequest => send_persistence_request(jetstream).await?,
        Commands::SendCreateShortRequest => send_create_short_request(jetstream).await?,
        Commands::SendShortCreatedResponse => send_short_created_response(jetstream).await?,
        Commands::SendRetrieveShortCommand => send_retrieve_short_command(jetstream).await?,
        Commands::ConsumeShortCreatedResponse => {
            consume_short_created_response(nats_url.to_string()).await?
        }
    };

    Ok(())
}
