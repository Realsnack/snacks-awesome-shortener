use snacks_awesome_shortener::config::Config;
use snacks_awesome_shortener::{build_app, setup_logging, run};

#[tokio::main]
async fn main() {
    setup_logging();
    let config = Config::from_env();
    let app = build_app(&config).await;
    run(app, config).await;
}
