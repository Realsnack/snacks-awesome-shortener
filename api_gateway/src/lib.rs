use async_nats::jetstream::Message;
use axum::Router;
use futures_util::TryStreamExt;
use common::messaging_config::MessagingConfig;
use common::nats_utils::create_consumer;
use config::Config;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use tracing::{debug, error, info};

pub mod config;

pub async fn build_app(config: &Config) -> Router {
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    const NAME: &str = env!("CARGO_PKG_NAME");

    info!(
        "Starting {} v{} on address: {}:{}",
        NAME, VERSION, config.app_address, config.app_port
    );

    Router::new().layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()))
}

pub async fn run(app: Router, config: Config) {
    match tokio::net::TcpListener::bind(format!("{}:{}", config.app_address, config.app_port)).await
    {
        Ok(listener) => {
            axum::serve(listener, tower::make::Shared::new(app))
                .await
                .unwrap();
        }
        Err(e) => {
            error!("Couldn't start app due to error: '{}'", e);
        }
    };
}

pub async fn run_consumer(consumer_config: MessagingConfig) -> Result<(), async_nats::Error> {
    let mut consumer_stream = create_consumer(&consumer_config).await?;

    while let Ok(Some(message)) = consumer_stream.try_next().await {
        process_message(&message).await;
        message.ack().await?;
    }

    Ok(())
}

pub async fn process_message(message: &Message) {
    debug!("Message payload: {:?}", &message.message);
}
