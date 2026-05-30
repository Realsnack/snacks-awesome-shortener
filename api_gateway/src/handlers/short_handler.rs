use crate::state::AppState;
use axum::Json;
use axum::body::Body;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Redirect, Response};
use common::TypeString;
use common::models::messaging::{
    CreateShortCommand, RetrieveShortCommand, ShortCreatedEvent, ShortRetrievedEvent,
};
use common::models::rest::CreateShortRequest;
use common::nats_utils::create_common_headers;
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

    let correlation_id = get_correlation_id_or_generate(headers);

    let (tx, rx) = oneshot::channel();
    state.pending.insert(correlation_id.clone(), tx);

    let short_command: CreateShortCommand = short_request.into();
    let mut nats_headers =
        create_common_headers(short_command.type_as_string(), correlation_id.clone());
    nats_headers.insert("response_subject", "api_gateway::response");

    match state
        .client
        .publish_with_headers(
            "shorts_service::request",
            nats_headers,
            short_command.to_proto().encode_to_vec().into(),
        )
        .await
    {
        Ok(_) => info!(
            "Sent {} with X-Correlation-Id '{}'",
            short_command.type_as_string(),
            correlation_id.clone()
        ),
        Err(e) => info!(
            "Failed to send message {} with X-Correlation-Id '{}' due to error: {}",
            short_command.type_as_string(),
            correlation_id.clone(),
            e
        ),
    }

    match tokio::time::timeout(std::time::Duration::from_secs(10), rx).await {
        Ok(Ok(response)) => {
            debug!("Obtained response headers: {:?}", response.headers);
            debug!("Obtained response message: {:?}", response.message);
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
            StatusCode::REQUEST_TIMEOUT.into_response()
        }
    }
}

pub async fn handle_short_get(
    State(state): State<AppState>,
    headers: axum::http::header::HeaderMap,
    Path(short_url): Path<String>,
) -> Response {
    let correlation_id = get_correlation_id_or_generate(headers);

    let (tx, rx) = oneshot::channel();
    state.pending.insert(correlation_id.clone(), tx);

    let retrieve_short_command = RetrieveShortCommand::new(short_url);
    let mut nats_headers = create_common_headers(
        retrieve_short_command.type_as_string(),
        correlation_id.clone(),
    );
    nats_headers.insert("response_subject", "api_gateway::response");

    // TODO: Make into it's own function
    match state
        .client
        .publish_with_headers(
            "data_persistor::request",
            nats_headers,
            retrieve_short_command.to_proto().encode_to_vec().into(),
        )
        .await
    {
        Ok(_) => info!(
            "Sent {} with X-Correlation-Id '{}'",
            retrieve_short_command.type_as_string(),
            correlation_id.clone()
        ),
        Err(e) => info!(
            "Failed to send message {} with X-Correlation-Id '{}' due to error: {}",
            retrieve_short_command.type_as_string(),
            correlation_id.clone(),
            e
        ),
    }

    match tokio::time::timeout(std::time::Duration::from_secs(10), rx).await {
        Ok(Ok(response)) => {
            debug!("Obtained response headers: {:?}", response.headers);
            debug!("Obtained response message: {:?}", response.message);
            let decoded_payload =
                common::proto::messaging::v1::events::ShortRetrievedEvent::decode(
                    response.message.payload,
                )
                .unwrap();
            let retrieved_short_event = ShortRetrievedEvent::from(decoded_payload);
            debug!("Retrieved short: {:?}", retrieved_short_event);
            Response::builder()
                .status(StatusCode::OK)
                .body(Body::from(json!(retrieved_short_event.short).to_string()))
                .unwrap()
        }
        _ => {
            warn!("Timed out waiting for response of '{}'", correlation_id);
            StatusCode::REQUEST_TIMEOUT.into_response()
        }
    }
}

pub async fn handle_short_redirect(
    State(state): State<AppState>,
    headers: axum::http::header::HeaderMap,
    Path(short_url): Path<String>,
) -> Response {
    let correlation_id = get_correlation_id_or_generate(headers);

    let (tx, rx) = oneshot::channel();
    state.pending.insert(correlation_id.clone(), tx);

    let retrieve_short_command = RetrieveShortCommand::new(short_url);
    let mut nats_headers = create_common_headers(
        retrieve_short_command.type_as_string(),
        correlation_id.clone(),
    );
    nats_headers.insert("response_subject", "api_gateway::response");

    match state
        .client
        .publish_with_headers(
            "data_persistor::request",
            nats_headers,
            retrieve_short_command.to_proto().encode_to_vec().into(),
        )
        .await
    {
        Ok(_) => info!(
            "Sent {} with X-Correlation-Id '{}'",
            retrieve_short_command.type_as_string(),
            correlation_id.clone()
        ),
        Err(e) => info!(
            "Failed to send message {} with X-Correlation-Id '{}' due to error: {}",
            retrieve_short_command.type_as_string(),
            correlation_id.clone(),
            e
        ),
    }

    match tokio::time::timeout(std::time::Duration::from_secs(10), rx).await {
        Ok(Ok(response)) => {
            debug!("Obtained response headers: {:?}", response.headers);
            debug!("Obtained response message: {:?}", response.message);
            let decoded_payload =
                common::proto::messaging::v1::events::ShortRetrievedEvent::decode(
                    response.message.payload,
                )
                .unwrap();
            let retrieved_short_event = ShortRetrievedEvent::from(decoded_payload);
            debug!("Retrieved short: {:?}", retrieved_short_event);
            Redirect::temporary(retrieved_short_event.short.long_url.as_str()).into_response()
        }
        _ => {
            warn!("Timed out waiting for response of '{}'", correlation_id);
            StatusCode::REQUEST_TIMEOUT.into_response()
        }
    }
}

fn get_correlation_id_or_generate(headers: axum::http::HeaderMap) -> String {
    headers
        .get("X-Correlation-Id")
        .and_then(|v| v.to_str().ok())
        .map(str::to_owned)
        .unwrap_or_else(|| Uuid::new_v4().to_string())
}
