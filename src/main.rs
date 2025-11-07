mod handlers;
mod models;
mod routes;
mod services;
mod state;

use std::sync::Arc;
use tide::log::{debug, info};
use tide::Server;
use services::{RedisService, ShortsService};
use state::AppState;
use crate::routes::shorts_routes::init_short_routes;

#[async_std::main]
async fn main() -> tide::Result<()> {
    if cfg!(debug_assertions) {
        femme::with_level(femme::LevelFilter::Debug);
        debug!("Debug logging enabled");
    } else {
        femme::with_level(femme::LevelFilter::Info);
        info!("Info logging enabled");
    }

    let app_address = std::env::var("SAS_IP").unwrap_or_else(|_| {
        info!("SAS_IP not specified, using 0.0.0.0");
        String::from("0.0.0.0")
    });

    let app_port = std::env::var("SAS_PORT").unwrap_or_else(|_| {
        info!("SAS_PORT not specified, using port 8080");
        String::from("8080")
    });
    let app_listen = format!("{}:{}", app_address, app_port);

    let redis_url = std::env::var("REDIS_URL").unwrap_or_else(|_| {
        info!("REDIS_URL not specified, using 127.0.0.1:6379");
        String::from("redis::/127.0.0.1:6379")
    });

    let redis_client = redis::Client::open(redis_url)?;
    let redis_service = Arc::new(RedisService::new(redis_client));
    let shorts_service = Arc::new(ShortsService::new(redis_service.clone()));

    let state = AppState {
        shorts_service,
    };

    let mut app = Server::with_state(state);

    init_short_routes(&mut app);

    app.listen(app_listen).await?;
    Ok(())
}
