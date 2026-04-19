use async_nats::HeaderMap;
use async_nats::jetstream::Context;
use clap::{Parser, Subcommand};
use common::models::create_short_request::CreateShortRequest;
use common::models::persistence_request::PersistenceRequest;
use common::models::short_url::ShortUrl;
use common::setup_logging;
use std::time::SystemTime;
use futures_util::TryStreamExt;
use tracing::info;
use common::messaging_config::MessagingConfig;
use common::models::created_short_response::CreatedShortResponse;
use common::nats_utils::create_consumer;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    SendPersistenceRequest,
    SendCreateShortRequest,
    SendShortCreatedResponse,
    ConsumeShortCreatedResponse,
}

async fn setup_jetstream(nats_url: &str) -> Result<Context, async_nats::Error> {
    info!("Connecting to: '{}'", nats_url);
    let client = async_nats::connect(nats_url).await?;
    Ok(async_nats::jetstream::new(client))
}

async fn send_persistence_request(jetstream: Context) -> Result<(), async_nats::Error> {
    let short_url = ShortUrl::new("asdfgh".to_string(), "https://hltv.org".to_string(), 600);
    let data = PersistenceRequest::new(
        short_url,
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)?
            .as_secs(),
    );

    info!("Publishing message: {:?}", data);
    jetstream
        .publish("data_persistor::request", data.to_vec()?.into())
        .await?;
    jetstream.client().flush().await?;

    Ok(())
}

async fn send_create_short_request(jetstream: Context) -> Result<(), async_nats::Error> {
    let create_short_request = CreateShortRequest::new(
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)?
            .as_secs(),
        "https://hltv.org/".into(),
        3600,
    );
    let mut headers = HeaderMap::new();
    headers.insert("message_type", "CreateShortRequest");

    info!("Publishing message: {:?}", create_short_request);
    jetstream
        .publish_with_headers(
            "shorts_service::request",
            headers,
            create_short_request.to_vec()?.into(),
        )
        .await?;
    jetstream.client().flush().await?;

    Ok(())
}

async fn send_short_created_response(jetstream: Context) -> Result<(), async_nats::Error> {
    let short = ShortUrl::new("short_url".into(), "some_long_url".into(), 1);
    let created_short = CreatedShortResponse::new(short, "test_tool".into());
    let mut headers = HeaderMap::new();
    headers.insert("message_type", "CreatedShortResponse");
    headers.insert("correlation_id", "test-tool");

    info!("Publishing message: {:?}", created_short);
    jetstream
        .publish_with_headers(
            "api_gateway::response",
            headers,
            created_short.to_vec()?.into(),
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
        10000
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
        Commands::ConsumeShortCreatedResponse => consume_short_created_response(nats_url.to_string()).await?,
    };

    Ok(())
}
