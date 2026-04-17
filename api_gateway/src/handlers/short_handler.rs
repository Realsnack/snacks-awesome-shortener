use axum::body::Body;
use crate::state::AppState;
use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde_json::json;
use common::models::create_short_request::CreateShortRequest;
use tokio::sync::oneshot;
use tracing::{debug, info, warn};
use uuid;
use uuid::Uuid;
use common::models::created_short_response::CreatedShortResponse;

pub async fn handle_short_post(
    State(state): State<AppState>,
    headers: axum::http::header::HeaderMap,
    Json(short_request): Json<CreateShortRequest>, // Replace with better model
) -> Response {
    debug!("Short request: '{:?}'", short_request);

    let correlation_id = headers
        .get("X-Correlation-Id")
        .and_then(|v| v.to_str().ok())
        .map(str::to_owned)
        .unwrap_or_else(|| Uuid::new_v4().to_string());

    let (tx, rx) = oneshot::channel();
    state.pending.insert(correlation_id.clone(), tx);

    let mut nats_headers = async_nats::HeaderMap::new();
    nats_headers.insert("correlation_id", correlation_id.clone());
    nats_headers.insert("message_type", "CreateShortRequest");
    nats_headers.insert("response_subject", "api_gateway::response");
    info!(
        "Sending request with X-Correlation-Id '{}'",
        correlation_id.clone()
    );
    let _client = state
        .client
        .publish_with_headers(
            "shorts_service::request",
            nats_headers,
            short_request.to_vec().unwrap().into(),
        )
        .await;

    match tokio::time::timeout(std::time::Duration::from_secs(10), rx).await {
        Ok(Ok(response)) => {
            debug!("Response headers: {:?}", response.headers);
            debug!("Response message: {:?}", response.message);
            let created_short = CreatedShortResponse::from_bytes(&response.message.payload).unwrap();
            debug!("Created short: {:?}", created_short);
            Response::builder()
                .status(StatusCode::OK)
                .body(Body::from(json!(created_short.short).to_string()))
                .unwrap()
        }
        _ => {
            warn!("Timed out waiting for response of '{}'", correlation_id);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }

}
