use axum::body::Body;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde_json::{json};
use tracing::info;
use crate::state::AppState;

pub async fn handle_health(State(state): State<AppState>) -> Response {
    info!("Checking application health");
    let services_health = state.health_service.get_services_health().await;

    Response::builder()
        .status(StatusCode::OK)
        .body(Body::from(json!(services_health).to_string()))
        .unwrap()
}

pub async fn handle_service_health(Path(service): Path<String>, State(state): State<AppState>) -> Response {
    info!("Checking service '{}' health", service);

    let service_status = match service.as_str() {
        "redis" => state.health_service.get_redis_health().await,
        "mongo" => state.health_service.get_mongo_health().await,
        _ => {
            info!("Service {} not supported", service);
            return StatusCode::NOT_FOUND.into_response();
        }
    };

    Response::builder()
        .status(StatusCode::OK)
        .body(Body::from(json!(service_status).to_string()))
        .unwrap()
}
