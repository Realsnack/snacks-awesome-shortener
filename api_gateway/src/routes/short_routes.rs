use axum::Router;
use axum::routing::post;
use crate::handlers::short_handler;
use crate::state::AppState;

pub fn shorts_routes() -> Router<AppState> {
    Router::new()
        .route("/short", post(short_handler::handle_short_post))
}