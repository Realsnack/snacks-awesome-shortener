mod handlers;
mod models;
mod routes;
mod services;
mod state;

use std::sync::Arc;
use axum::Router;
use services::{MongoService, RedisService, ShortsService};
use mongodb::Client;
use mongodb::options::{ClientOptions, ServerApi, ServerApiVersion};
use routes::{root_routes, shorts_routes};
use state::AppState;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use tracing::{debug, error, info};

#[tokio::main]
async fn main() {
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

    let app_address = std::env::var("SAS_IP").unwrap_or_else(|_| {
        debug!("SAS_IP not specified, using 0.0.0.0");
        String::from("0.0.0.0")
    });

    let app_port = std::env::var("SAS_PORT").unwrap_or_else(|_| {
        debug!("SAS_PORT not specified, using port 8080");
        String::from("8080")
    });

    info!(
        "Starting {} v{} on address: {}:{}",
        std::env::var("CARGO_PKG_NAME").unwrap_or("snacks-awesome-shortener".to_string()),
        std::env::var("CARGO_PKG_VERSION").unwrap_or("X.X".to_string()),
        app_address,
        app_port
    );

    let redis_url = std::env::var("REDIS_URL").unwrap_or_else(|_| {
        info!("REDIS_URL not specified, using 'redis://127.0.0.1:6379'");
        String::from("redis://127.0.0.1:6379")
    });

    let mongo_url = std::env::var("MONGO_URL").unwrap_or_else(|_| {
        info!("MONGO_URL not specified, using 'mongodb://127.0.0.1:27017'");
        String::from("mongodb://127.0.0.1:27017")
    });
    let mut client_options = match ClientOptions::parse(mongo_url).await {
        Ok(opts) => opts,
        Err(e) => {
            error!("Received error while construction ClientOptions: '{}'", e);
            panic!()
        }
    };
    let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
    client_options.server_api = Some(server_api);

    let redis_client = redis::Client::open(redis_url).unwrap();
    let redis_service = Arc::new(RedisService::new(redis_client));
    let mongo_client = Client::with_options(client_options).unwrap();
    let mongo_service = Arc::new(MongoService::new(mongo_client));
    let shorts_service = Arc::new(ShortsService::new(redis_service, mongo_service));

    let state = AppState {
        shorts_service,
    };

    let app = Router::new()
        .merge(root_routes::root_routes())
        .merge(shorts_routes::shorts_routes())
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
        )
        .with_state(state);

    match tokio::net::TcpListener::bind("0.0.0.0:8080").await {
        Ok(listener) => {
            axum::serve(listener, tower::make::Shared::new(app)).await.unwrap();
        }
        Err(e) => {
            error!("Couldn't start app due to error: '{}'", e);
        }
    };
}
