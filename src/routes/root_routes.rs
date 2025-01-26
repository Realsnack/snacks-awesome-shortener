pub fn init_root_routes(app: &mut tide::Server<()>) {
    app.at("/").get(crate::handlers::root_handler::handle_get);
} 