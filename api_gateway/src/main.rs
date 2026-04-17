use api_gateway::config::Config;
use api_gateway::{build_app, build_state, run, run_consumer};
use common::setup_logging;
use common::messaging_config::MessagingConfig;

#[tokio::main]
async fn main() {
    setup_logging();
    let api_config = Config::from_env();
    let consumer_config = MessagingConfig::from_env(env!("CARGO_PKG_NAME").to_string());

    let state = build_state(&consumer_config).await;
    let app = build_app(&api_config, state.clone()).await;

    let consumer_task = tokio::spawn(run_consumer(consumer_config, state));
    let api_task = tokio::spawn(run(app, api_config));

    let _ = tokio::try_join!(api_task, consumer_task);
}
