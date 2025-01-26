mod handlers;
mod models;
mod routes;

use routes::root_routes::init_root_routes;

use tide::log::{debug, info};

#[tokio::main]
async fn main() -> tide::Result<()> {
    if cfg!(debug_assertions) {
        femme::with_level(femme::LevelFilter::Debug);
        debug!("Debug logging enabled");
    } else {
        femme::with_level(femme::LevelFilter::Info);
        info!("Info logging enabled");
    }
    let mut app = tide::new();

    init_root_routes(&mut app);

    app.listen("0.0.0.0:8080").await?;
    Ok(())
}
