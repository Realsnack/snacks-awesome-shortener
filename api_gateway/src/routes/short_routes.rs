use crate::handlers::short_handler;
use crate::state::AppState;
use axum::Router;
use axum::routing::{get, post};

pub fn shorts_routes() -> Router<AppState> {
    Router::new()
        .route("/short/{short_url}", get(short_handler::handle_short_get))
        .route("/short", post(short_handler::handle_short_post))
}
