use tracing::info;
use api_gateway::config::Config;
use api_gateway::{build_app, run, run_consumer};
use common::setup_logging;
use common::messaging_config::MessagingConfig;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    setup_logging();
    let instance_uuid = Uuid::new_v4().to_string();
    let instance_id = instance_uuid.split("-").last().unwrap();
    info!("Instance id: {:?}", instance_id);
    let mut consumer_config = MessagingConfig::from_env(env!("CARGO_PKG_NAME").to_string());
    consumer_config.request_stream = format!("{}::{}", consumer_config.response_stream, instance_id);
    let consumer_task = tokio::spawn(run_consumer(consumer_config));
    let api_config = Config::from_env();
    let app = build_app(&api_config).await;
    let api_task = tokio::spawn(run(app, api_config));

    let _ = tokio::try_join!(api_task, consumer_task);
}
