pub mod models;
pub mod config;
pub mod nats_utils;

pub fn setup_logging() {
    if cfg!(debug_assertions) {
        tracing_subscriber::fmt()
            .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
            .init();
    } else {
        tracing_subscriber::fmt()
            .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
            .json()
            .init();
    }
}