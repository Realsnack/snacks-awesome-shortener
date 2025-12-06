use axum::body::Body;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::Response;
use serde_json::{json};
use tracing::info;
use crate::state::AppState;

pub async fn handle_health(State(state): State<AppState>) -> Response {
    info!("Checking application health");
    let services_health = state.health_service.get_services_health().await;

    Response::builder()
        .status(StatusCode::OK)
        .body(Body::from(json!({"status": "UP", "services": services_health}).to_string()))
        .unwrap()
}
