mod handlers;
mod models;
mod routes;
mod services;

use std::ffi::OsString;

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

    let app_address = match std::env::var("SAS_IP") {
        Ok(address) => address,
        Err(_) => {
            info!("SAS_IP not specified, using 0.0.0.0");
            String::from("0.0.0.0")
        }
    };

    let app_port = match std::env::var("SAS_PORT") {
        Ok(port) => port,
        Err(_) => {
            info!("SAS_PORT not specified, using port 8080");
            String::from("8080")
        }
    };

    let mut app = tide::new();

    init_root_routes(&mut app);

    let app_listen = format!("{}:{}", app_address, app_port);

    app.listen(app_listen).await?;
    Ok(())
}
