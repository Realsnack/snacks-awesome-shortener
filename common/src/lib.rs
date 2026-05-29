pub mod config;
pub mod models;
pub mod nats_utils;
pub mod pg_utils;
pub mod proto;
pub mod traits;
pub use config::Config;
pub use traits::TypeString;

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
