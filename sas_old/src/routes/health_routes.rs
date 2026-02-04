use axum::Router;
use axum::routing::get;
use crate::handlers::health_handler;
use crate::state::AppState;

pub fn health_routes() -> Router<AppState> {
    Router::new()
        .route("/health", get(health_handler::handle_health))
        .route("/health/{service}", get(health_handler::handle_service_health))
}
