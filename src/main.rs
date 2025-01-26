use tide::{log::{debug, info}, Redirect, Request};

mod models;

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
    app.at("/").get(hello);
    app.listen("0.0.0.0:8080").await?;
    Ok(())
}

async fn hello(mut _req: Request<()>) -> tide::Result<String> {
    Ok("{\"Hello\": \"World!\"}\n".into())
}
