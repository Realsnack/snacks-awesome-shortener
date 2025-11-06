use crate::handlers;
use crate::state::AppState;

pub fn init_short_routes(app: &mut tide::Server<AppState>) {
    app.at("/:short").get(handlers::short_handler::handle_short_redirect);
    app.at("/short/:short").get(handlers::short_handler::handle_short_get);
    app.at("/short").post(handlers::short_handler::handle_short_post);
}