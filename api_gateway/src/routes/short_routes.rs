use crate::handlers::short_handler::{self, handle_short_redirect};
use crate::state::AppState;
use axum::Router;
use axum::routing::{get, post};

pub fn shorts_routes() -> Router<AppState> {
    Router::new()
        .route("/{short_url}", get(handle_short_redirect))
        .route("/short/{short_url}", get(short_handler::handle_short_get))
        .route("/short", post(short_handler::handle_short_post))
}
