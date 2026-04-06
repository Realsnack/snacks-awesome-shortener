use std::sync::Arc;
use crate::routes::root_routes::root_routes;
use axum::Router;
use dashmap::DashMap;
use common::messaging_config::MessagingConfig;
use common::nats_utils::{create_pull_consumer, get_stream};
use config::Config;
use futures_util::{StreamExt, TryStreamExt};
use tokio::sync::oneshot;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use tracing::{debug, error, info};
use crate::state::AppState;

pub mod config;
pub mod handlers;
pub mod routes;
pub mod services;
pub mod state;

pub async fn build_app(config: &Config) -> Router {
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    const NAME: &str = env!("CARGO_PKG_NAME");

    info!(
        "Starting {} v{} on address: {}:{}",
        NAME, VERSION, config.app_address, config.app_port
    );

    Router::new()
        .merge(root_routes())
        .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()))
}

pub async fn build_state(config: &MessagingConfig) -> AppState {
    let client = async_nats::connect(&config.nats_url).await.unwrap();

    let pending_map: DashMap<String, oneshot::Sender<String>> = DashMap::new();
    let pending = Arc::new(pending_map);

    AppState {
        pending,
        client,
    }
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

pub async fn run_consumer(consumer_config: MessagingConfig, state: AppState) -> Result<(), async_nats::Error> {
    let jetstream = async_nats::jetstream::new(state.client);
    let stream = get_stream(&jetstream, consumer_config.response_stream.clone(), consumer_config.request_stream_max_messages).await?;
    let consumer = create_pull_consumer(stream, consumer_config.consumer_name, consumer_config.response_stream).await?;
    let mut messages = consumer.messages().await?.take(1);

    while let Ok(Some(message)) = messages.try_next().await {
        debug!("Message payload: {:?}", &message.message);
        message.ack().await?;
    }

    Ok(())
}
