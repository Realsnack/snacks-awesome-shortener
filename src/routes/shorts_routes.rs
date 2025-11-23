use crate::handlers::short_handler;
use axum::Router;
use axum::routing::{get, post};
use crate::state::AppState;

pub fn shorts_routes() -> Router<AppState> {
    Router::new()
        .route("/{short_url}", get(short_handler::handle_short_redirect))
        .route("/short/{short_url}", get(short_handler::handle_short_get))
        .route("/short", post(short_handler::handle_short_post))
}
