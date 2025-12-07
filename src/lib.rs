use std::sync::Arc;
use axum::Router;
use mongodb::Client;
use mongodb::options::{ClientOptions, ServerApi, ServerApiVersion};
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use tracing::{error, info};
use config::Config;
use crate::routes::{health_routes, root_routes, shorts_routes};
use crate::services::{MongoService, RedisService, ShortsService};
use crate::services::health_service::HealthService;
use crate::state::AppState;

pub mod handlers;
pub mod models;
pub mod routes;
pub mod services;
pub mod state;

pub mod config;

pub fn setup_logging() {
    if cfg!(debug_assertions) {
        tracing_subscriber::fmt()
            .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
            // .json()
            .init();
    } else {
        tracing_subscriber::fmt()
            .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
            .json()
            .init();
    }
}

pub async fn build_app(config: &Config) -> Router {
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    const NAME: &str = env!("CARGO_PKG_NAME");

    info!(
        "Starting {} v{} on address: {}:{}",
        NAME, VERSION, config.app_address, config.app_port
    );

    let mut client_options = match ClientOptions::parse(config.mongo_url.clone()).await {
        Ok(opts) => opts,
        Err(e) => {
            error!("Received error while construction ClientOptions: '{}'", e);
            panic!()
        }
    };
    let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
    client_options.server_api = Some(server_api);

    let redis_client = redis::Client::open(config.redis_url.clone()).unwrap();
    let redis_service = Arc::new(RedisService::new(redis_client));
    let mongo_client = Client::with_options(client_options).unwrap();
    let mongo_service = Arc::new(MongoService::new(mongo_client));
    let health_service = Arc::new(HealthService::new(redis_service.clone(), mongo_service.clone()));
    let shorts_service = Arc::new(ShortsService::new(redis_service, mongo_service));

    let state = AppState { shorts_service, health_service };

    Router::new()
        .merge(root_routes::root_routes())
        .merge(health_routes::health_routes())
        .merge(shorts_routes::shorts_routes())
        .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()))
        .with_state(state)
}

pub async fn run(app: Router, config: Config) {
    match tokio::net::TcpListener::bind(format!("{}:{}", config.app_address, config.app_port)).await {
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
