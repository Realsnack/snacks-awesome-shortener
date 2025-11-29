use crate::models::short_url::ShortUrl;
use crate::state::AppState;
use axum::body::{Body, Bytes};
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Redirect, Response};
use serde_json::{json, Value};
use tracing::{debug, info};

pub async fn handle_short_redirect(
    State(state): State<AppState>,
    Path(short_url): Path<String>,
) -> Response {
    info!("Searching for short_url: '{}'", short_url);
    let service = state.shorts_service;

    match service.get_long_url(short_url).await {
        Some(long_url) => {
            let short_url_object: ShortUrl = serde_json::from_str(&long_url).unwrap();
            Redirect::temporary(short_url_object.long_url.as_str()).into_response()
        }
        None => (StatusCode::NOT_FOUND).into_response(),
    }
}

pub async fn handle_short_get(
    State(state): State<AppState>,
    Path(short_url): Path<String>,
) -> Response {
    debug!("Searching for short_url: '{}'", short_url);
    let service = state.shorts_service;

    match service.get_long_url(short_url).await {
        Some(short_url) => Response::builder()
            .status(StatusCode::OK)
            .body(Body::from(short_url))
            .unwrap(),
        None => {
            info!("short_url not found in redis");
            StatusCode::NOT_FOUND.into_response()
        }
    }
}

pub async fn handle_short_post(
    State(state): State<AppState>,
    body_bytes: Bytes,
) -> Response {
    let body: Value = match serde_json::from_slice(&body_bytes) {
        Ok(v) => v,
        Err(_) => {
            return Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(Body::from("{\"reason\": \"No 'url' in request body\"}"))
                .unwrap();
        }
    };

    debug!("Short post body: '{}'", body);
    let long_url = match body["url"].as_str() {
        None => {
            info!("No 'url' in request body: {}", body);
            return Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(Body::from("{\"reason\": \"No 'url' in request body\"}"))
                .unwrap();
        }
        Some(url) => url,
    };

    let service = state.shorts_service;
    match service.generate_short_url(long_url.into()).await {
        None => StatusCode::NOT_FOUND.into_response(),
        Some(short_url) => Response::builder()
            .status(StatusCode::OK)
            .body(Body::from(json!(short_url).to_string()))
            .unwrap(),
    }
}
