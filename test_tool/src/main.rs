use std::time::SystemTime;
use clap::{Parser, Subcommand};
use common::models::persistence_request::PersistenceRequest;
use common::models::short_url::ShortUrl;
use common::setup_logging;
use tracing::info;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    SendPersistenceRequest
}

async fn send_persistence_request() -> Result<(), async_nats::Error> {
    setup_logging();

    let nats_url = "localhost:4222";
    info!("Connecting to: '{}'", nats_url);
    let client = async_nats::connect(nats_url).await?;
    let jetstream = async_nats::jetstream::new(client);

    let short_url = ShortUrl::new(
        "asdfgh".to_string(),
        "https://hltv.org".to_string(),
        600);
    let data = PersistenceRequest::new(
        short_url,
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)?
            .as_secs());

    let _request_stream = jetstream
        .get_or_create_stream(async_nats::jetstream::stream::Config {
            name: "data_persistor::request".to_string(),
            max_messages: 1_000,
            ..Default::default()
        }).await?;

    info!("Publishing message: {:?}", data);
    jetstream.publish("data_persistor::request", data.to_vec()?.into()).await?;
    jetstream.client().flush().await?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), async_nats::Error> {
    let args = Args::parse();

    info!("Action chosen {:?}", args.command);

    match args.command {
        Commands::SendPersistenceRequest => send_persistence_request().await?
    };

    Ok(())
}
