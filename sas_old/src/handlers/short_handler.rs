use crate::models::short_url::ShortUrl;
use crate::state::AppState;
use axum::body::{Body};
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Redirect, Response};
use serde_json::{json};
use tracing::{debug, info};
use crate::models::short_request::ShortRequest;

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
    Json(short_request): Json<ShortRequest>,
) -> Response {
    debug!("Short request: '{:?}'", short_request);

    let service = state.shorts_service;
    match service.generate_short_url(short_request.url).await {
        None => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        Some(short_url) => Response::builder()
            .status(StatusCode::OK)
            .body(Body::from(json!(short_url).to_string()))
            .unwrap(),
    }
}
