use api_gateway::config::Config;
use api_gateway::{build_app, run};
use common::setup_logging;

#[tokio::main]
async fn main() {
    setup_logging();
    let config = Config::from_env();
    let app = build_app(&config).await;
    run(app, config).await;
}