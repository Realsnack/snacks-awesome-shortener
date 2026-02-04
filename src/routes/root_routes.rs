use axum::Router;
use axum::routing::get;
use crate::handlers::root_handler;
use crate::state::AppState;

pub fn root_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(root_handler::handle_root_get))
        .route("/greet/{name}", get(root_handler::handle_greet))
}
