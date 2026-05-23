use crate::state::AppState;
use axum::Json;
use axum::body::Body;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use common::TypeString;
use common::models::messaging::{CreateShortCommand, ShortCreatedEvent};
use common::models::rest::CreateShortRequest;
use prost::Message;
use serde_json::json;
use tokio::sync::oneshot;
use tracing::{debug, info, warn};
use uuid;
use uuid::Uuid;

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

    let short_command: CreateShortCommand = short_request.into();
    let mut nats_headers = async_nats::HeaderMap::new();
    nats_headers.insert("correlation_id", correlation_id.clone());
    nats_headers.insert("message_type", short_command.type_as_string());
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
            short_command.to_proto().encode_to_vec().into(),
        )
        .await;

    match tokio::time::timeout(std::time::Duration::from_secs(10), rx).await {
        Ok(Ok(response)) => {
            debug!("Response headers: {:?}", response.headers);
            debug!("Response message: {:?}", response.message);
            let decoded_payload = common::proto::messaging::v1::events::ShortCreatedEvent::decode(
                response.message.payload,
            )
            .unwrap();
            let created_short = ShortCreatedEvent::from(decoded_payload);
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
