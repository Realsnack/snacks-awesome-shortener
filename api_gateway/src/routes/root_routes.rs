use axum::Router;
use axum::routing::get;
use crate::handlers::root_handler;

pub fn root_routes() -> Router {
    Router::new()
        .route("/", get(root_handler::handle_root_get))
}